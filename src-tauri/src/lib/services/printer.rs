use crate::lib::models::*;
use crate::lib::db::DB;
use rusqlite::params;
use std::io::Write;
use std::net::TcpStream;
use std::time::Duration;
use std::fs::OpenOptions;
use log::{info, error, warn};
use rusb::{self, DeviceHandle, UsbContext};
use encoding_rs::GBK;

#[cfg(windows)]
use crate::lib::services::printer_bitmap::print_receipt_bitmap;

pub const ESC: u8 = 0x1B;
pub const FS: u8 = 0x1C;
pub const GS: u8 = 0x1D;
const LF: u8 = 0x0A;
const DEBUG_LOG_PATH: &str = "/Users/mrzat/Desktop/billiard-manager/.cursor/debug-eb248b.log";

fn debug_log(run_id: &str, hypothesis_id: &str, location: &str, message: &str, data: serde_json::Value) {
    let payload = serde_json::json!({
        "sessionId": "eb248b",
        "runId": run_id,
        "hypothesisId": hypothesis_id,
        "location": location,
        "message": message,
        "data": data,
        "timestamp": chrono::Utc::now().timestamp_millis(),
    });
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(DEBUG_LOG_PATH) {
        let _ = writeln!(f, "{}", payload);
    }
}

fn esc_pos_init() -> Vec<u8> {
    let mut buf = vec![ESC, b'@'];
    // 进入中文模式: FS &
    buf.extend_from_slice(&[FS, b'&']);
    // 设置汉字字符集为 GB18030 (可选，增强兼容性)
    // ESC R 选择国际字符集，但不影响中文模式
    buf
}

fn esc_pos_align_center() -> Vec<u8> {
    vec![ESC, b'a', 0x01]
}

fn esc_pos_align_left() -> Vec<u8> {
    vec![ESC, b'a', 0x00]
}

#[allow(dead_code)]
fn esc_pos_align_right() -> Vec<u8> {
    vec![ESC, b'a', 0x02]
}

fn esc_pos_bold_on() -> Vec<u8> {
    vec![ESC, b'E', 0x01]
}

fn esc_pos_bold_off() -> Vec<u8> {
    vec![ESC, b'E', 0x00]
}

fn esc_pos_text(text: &str) -> Vec<u8> {
    // 将 UTF-8 编码的中文转为 GBK 编码，热敏打印机只认 GBK
    let (bytes, _, _) = GBK.encode(text);
    let mut result = bytes.to_vec();
    result.push(LF);
    result
}

#[allow(dead_code)]
fn esc_pos_text_no_lf(text: &str) -> Vec<u8> {
    let (bytes, _, _) = GBK.encode(text);
    bytes.to_vec()
}

fn esc_pos_feed(n: u8) -> Vec<u8> {
    vec![ESC, b'd', n]
}

fn esc_pos_cut() -> Vec<u8> {
    vec![GS, b'V', 0x00]
}

fn esc_pos_divider(width: i32) -> Vec<u8> {
    let line = "=".repeat(width as usize / 2);
    let (bytes, _, _) = GBK.encode(&line);
    let mut result = bytes.to_vec();
    result.push(LF);
    result
}

fn esc_pos_left_right(left: &str, right: &str, width: i32) -> Vec<u8> {
    // GBK编码下中文字符占2字节，ASCII占1字节
    // 用字符数近似计算显示宽度（中文2列宽，ASCII 1列宽）
    let display_width = |s: &str| -> usize {
        s.chars().map(|c| if c.is_ascii() { 1 } else { 2 }).sum()
    };
    let max_display = (width as usize / 2).saturating_sub(2);

    let left_display = if display_width(left) > max_display {
        // 截断，保留前半部分
        let mut s = String::new();
        let mut w = 0;
        for c in left.chars() {
            let cw = if c.is_ascii() { 1 } else { 2 };
            if w + cw + 2 > max_display { break; }
            s.push(c);
            w += cw;
        }
        s.push_str("..");
        s
    } else {
        left.to_string()
    };

    let right_display = if display_width(right) > max_display {
        // 截断，保留后半部分
        let mut s = String::new();
        let mut w = 0;
        for c in right.chars().rev() {
            let cw = if c.is_ascii() { 1 } else { 2 };
            if w + cw + 2 > max_display { break; }
            s.push(c);
            w += cw;
        }
        format!("..{}", s.chars().rev().collect::<String>())
    } else {
        right.to_string()
    };

    let left_w = display_width(&left_display);
    let right_w = display_width(&right_display);
    let padding = (width as usize / 2).saturating_sub(left_w + right_w);
    let mut line = left_display;
    line.push_str(&" ".repeat(padding));
    line.push_str(&right_display);
    let (bytes, _, _) = GBK.encode(&line);
    let mut result = bytes.to_vec();
    result.push(LF);
    result
}

fn generate_receipt_bytes(req: &PrintReceiptRequest, paper_width: i32) -> Vec<u8> {
    let mut buf = Vec::new();

    buf.extend(esc_pos_init());
    buf.extend(esc_pos_align_center());
    buf.extend(esc_pos_bold_on());
    buf.extend(esc_pos_text(&req.shop_name));
    buf.extend(esc_pos_bold_off());

    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    buf.extend(esc_pos_text(&now));

    buf.extend(esc_pos_divider(paper_width));

    buf.extend(esc_pos_align_left());

    if let Some(ref order_no) = req.order_no {
        buf.extend(esc_pos_left_right("订单号:", order_no, paper_width));
    }
    if let Some(ref table_name) = req.table_name {
        buf.extend(esc_pos_left_right("球桌:", table_name, paper_width));
    }
    if let Some(ref member_name) = req.member_name {
        buf.extend(esc_pos_left_right("会员:", member_name, paper_width));
    }
    if let Some(ref start_time) = req.start_time {
        buf.extend(esc_pos_left_right("开始时间:", start_time, paper_width));
    }
    if let Some(ref end_time) = req.end_time {
        buf.extend(esc_pos_left_right("结束时间:", end_time, paper_width));
    }
    if let Some(duration) = req.duration_minutes {
        buf.extend(esc_pos_left_right("时长:", &format!("{}分钟", duration), paper_width));
    }

    buf.extend(esc_pos_divider(paper_width));

    if req.receipt_type == "sale" {
        if let Some(ref items) = req.items {
            for item in items {
                let item_line = format!("{} x{}  ¥{:.2}", item.name, item.quantity, item.price);
                buf.extend(esc_pos_text(&item_line));
            }
            buf.extend(esc_pos_divider(paper_width));
        }
    }

    buf.extend(esc_pos_left_right("合计:", &format!("¥{:.2}", req.total_amount), paper_width));

    if let Some(discount) = req.discount_amount {
        if discount > 0.0 {
            buf.extend(esc_pos_left_right("优惠:", &format!("-¥{:.2}", discount), paper_width));
        }
    }
    if let Some(deposit) = req.deposit {
        if deposit > 0.0 {
            buf.extend(esc_pos_left_right("押金:", &format!("-¥{:.2}", deposit), paper_width));
        }
    }

    buf.extend(esc_pos_divider(paper_width));
    buf.extend(esc_pos_bold_on());
    buf.extend(esc_pos_left_right("实收:", &format!("¥{:.2}", req.final_amount), paper_width));
    buf.extend(esc_pos_bold_off());

    if let Some(ref payment_method) = req.payment_method {
        buf.extend(esc_pos_left_right("支付方式:", payment_method, paper_width));
    }

    buf.extend(esc_pos_divider(paper_width));
    buf.extend(esc_pos_align_center());
    buf.extend(esc_pos_text("感谢您的光临！"));
    buf.extend(esc_pos_text("欢迎下次再来"));

    buf.extend(esc_pos_feed(3));
    buf.extend(esc_pos_cut());

    buf
}

fn send_to_network_printer(ip: &str, port: i32, data: &[u8]) -> Result<(), String> {
    let addr = format!("{}:{}", ip, port);
    let mut stream = TcpStream::connect(&addr)
        .map_err(|e| format!("连接打印机失败 {}: {}", addr, e))?;
    stream.set_write_timeout(Some(std::time::Duration::from_secs(5)))
        .map_err(|e| format!("设置超时失败: {}", e))?;
    stream.write_all(data)
        .map_err(|e| format!("发送数据失败: {}", e))?;
    stream.flush()
        .map_err(|e| format!("刷新缓冲区失败: {}", e))?;
    Ok(())
}

fn send_to_serial_printer(_serial_port: &str, _baud_rate: i32, _data: &[u8]) -> Result<(), String> {
    Err("串口打印需要 serialport crate，暂未实现".to_string())
}

// ======== USB 打印机支持 ========

/// 常见USB热敏打印机VID/PID列表
const USB_PRINTER_DEVICES: &[(u16, u16, &str)] = &[
    // EPSON 热敏打印机
    (0x04B8, 0x0E15, "EPSON TM-T88V"),
    (0x04B8, 0x0E28, "EPSON TM-T20"),
    (0x04B8, 0x0E03, "EPSON TM-P2.01"),
    (0x04B8, 0x0E02, "EPSON TM-T70"),
    // 通用POS打印机 (Winbond/Nexa/Sinocan)
    (0x0416, 0x5011, "POS-58/80"),
    (0x0416, 0xAABB, "POS-58"),
    // SII/AVASYS 打印机
    (0x1446, 0x0601, "SII POS-58"),
    // Zjiang/ZJ 打印机
    (0x0483, 0x5740, "ZJ-5890"),
    // XPrinter
    (0x0483, 0x7584, "XPrinter XP-80C"),
    // Zebra 标签打印机
    (0x0A5F, 0x008B, "Zebra LP2844"),
    (0x0A5F, 0x0273, "Zebra GK420d"),
    (0x0A5F, 0x0275, "Zebra GX420d"),
    // Godex 标签打印机
    (0x0DD4, 0x015D, "Godex EZ-1100"),
    // TSC 标签打印机
    (0x1203, 0xA010, "TSC TTP-244"),
    // HP 打印机
    (0x03F0, 0x0002, "HP LaserJet"),
    // Canon 打印机
    (0x04A9, 0x0001, "Canon Printer"),
    // 通用打印机类标记 (用于匹配打印机类设备)
    (0x0000, 0x0000, "Generic USB Printer"),
];

/// USB打印机设备信息
#[allow(dead_code)]
struct UsbPrinterInfo {
    vendor_id: u16,
    product_id: u16,
    manufacturer: Option<String>,
    product: Option<String>,
}

/// 查找USB打印机设备
fn find_usb_printers() -> Result<Vec<UsbPrinterInfo>, String> {
    let context = rusb::GlobalContext::default();
    let devices = context.devices()
        .map_err(|e| format!("获取USB设备列表失败: {}", e))?;

    let mut printers = Vec::new();
    let mut scanned = 0usize;
    let mut known_match = 0usize;
    let mut class_match = 0usize;
    let mut interface_match = 0usize;

    for device in devices.iter() {
        let desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };
        scanned += 1;

        let vid = desc.vendor_id();
        let pid = desc.product_id();

        // 检查是否在已知打印机列表中
        let is_known_printer = USB_PRINTER_DEVICES.iter().any(|&(v, p, _)| {
            (v != 0 && p != 0) && (v == vid && p == pid)
        });

        // 检查设备类是否为打印机类 (bDeviceClass = 7)
        let is_printer_class = desc.class_code() == 0x07;

        // 检查子接口是否为打印机类
        let mut is_interface_printer_class = false;
        if let Ok(handle) = device.open() {
            if let Ok(config) = handle.active_configuration() {
                if let Ok(config_desc) = device.config_descriptor(config) {
                    for interface in config_desc.interfaces() {
                        for interface_desc in interface.descriptors() {
                            if interface_desc.class_code() == 0x07 {
                                is_interface_printer_class = true;
                                break;
                            }
                        }
                    }
                }
            }
        }

        if is_known_printer {
            known_match += 1;
        }
        if is_printer_class {
            class_match += 1;
        }
        if is_interface_printer_class {
            interface_match += 1;
        }

        // 只要匹配任何一个条件就添加
        if is_known_printer || is_printer_class || is_interface_printer_class {
            let handle = device.open().ok();
            let (manufacturer, product) = if let Some(ref h) = handle {
                let langs = h.read_languages(Duration::from_millis(500)).ok();
                if let Some(langs) = langs {
                    if let Some(lang) = langs.first() {
                        let mfr = h.read_manufacturer_string(*lang, &desc, Duration::from_millis(500)).ok();
                        let prod = h.read_product_string(*lang, &desc, Duration::from_millis(500)).ok();
                        (mfr, prod)
                    } else {
                        (None, None)
                    }
                } else {
                    (None, None)
                }
            } else {
                (None, None)
            };

            printers.push(UsbPrinterInfo {
                vendor_id: vid,
                product_id: pid,
                manufacturer,
                product,
            });
        }
    }

    info!("USB扫描: 扫描{}个设备, 已知匹配{}, 类匹配{}, 接口匹配{}, 打印机{}台",
        scanned, known_match, class_match, interface_match, printers.len());

    debug_log(
        "baseline",
        "H1",
        "src-tauri/src/lib/services/printer.rs:find_usb_printers",
        "usb_enumeration_summary",
        serde_json::json!({
            "scanned": scanned,
            "known_match": known_match,
            "class_match": class_match,
            "interface_match": interface_match,
            "printers_found": printers.len()
        }),
    );

    Ok(printers)
}

/// 枚举系统中的USB打印机
pub fn list_usb_printers() -> Vec<String> {
    match find_usb_printers() {
        Ok(printers) if !printers.is_empty() => printers.iter().map(|p| {
            format!(
                "{:04x}:{:04x} - {}",
                p.vendor_id,
                p.product_id,
                p.product.as_deref().unwrap_or("Unknown Printer")
            )
        }).collect(),
        Ok(_) => {
            info!("未发现USB打印机设备");
            vec![]
        }
        Err(e) => {
            warn!("枚举USB打印机失败: {}", e);
            vec![]
        }
    }
}

/// 通过Windows直接端口写入发送原始打印数据
#[allow(dead_code)]
#[cfg(windows)]
fn send_to_usb_printer(data: &[u8]) -> Result<(), String> {
    // 获取Windows第一个可用的真实打印机
    let (printer_name, _port_name) = get_windows_printer_info()
        .map_err(|e| format!("获取打印机信息失败: {}", e))?;

    if printer_name.is_empty() {
        return Err("未找到Windows打印机，请确保已安装打印机驱动".to_string());
    }

    info!("Printing via Windows: printer='{}', bytes={}", printer_name, data.len());

    // 保存原始数据到临时文件
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join(format!("billiard_print_{}.prn", std::process::id()));
    std::fs::write(&temp_file, data)
        .map_err(|e| format!("写入临时文件失败: {}", e))?;

    let printer_name_escaped = printer_name.replace("'", "''");
    let temp_file_path = temp_file.to_string_lossy().to_string();

    // 使用PowerShell执行打印
    let ps_script = format!(r#"
$ErrorActionPreference = 'Stop'
$printerName = '{}'
$tempFile = '{}'

$success = $false
$errorMsg = ''

try {{
    # 使用 Print 命令行工具
    $printExe = Join-Path $env:SystemRoot 'System32\spool\tools\print.exe'
    if (Test-Path $printExe) {{
        $process = Start-Process -FilePath $printExe -ArgumentList "/D:$printerName `"$tempFile`"" -Wait -PassThru -NoNewWindow
        if ($process.ExitCode -eq 0) {{
            $success = $true
        }}
    }}
    
    # 如果print.exe不可用，尝试直接复制文件到打印机端口
    if (-not $success) {{
        # 获取打印机的实际端口
        $port = (Get-Printer -Name $printerName -ErrorAction SilentlyContinue).PortName
        if ($port -and ($port -match '^(USB\d+|LPT\d+)')) {{
            $portPath = "\\.\$port"
            Copy-Item -Path $tempFile -Destination $portPath -Force
            $success = $true
        }}
    }}
    
    # 最后尝试使用 Raw print API
    if (-not $success) {{

        Add-Type -AssemblyName System.Drawing.Printing
        $srv = New-Object System.Drawing.Printing.PrintServer
        $queue = New-Object System.Drawing.Printing.PrintQueue($srv, $printerName, [System.Drawing.Printing.PrintSystemQueueOptions.Raw)
        
        # 读取数据
        $fs = [System.IO.File]::OpenRead($tempFile)
        $bytes = New-Object byte[] $fs.Length
        $fs.Read($bytes, 0, $fs.Length) | Out-Null
        $fs.Close()
        
        # 使用Raw模式写入
        $ptr = [Runtime.InteropServices.Marshal]::AllocHGlobal($bytes.Length)
        [Runtime.InteropServices.Marshal]::Copy($bytes, 0, $ptr, $bytes.Length)
        
        # 调用Win32 API
        Add-Type -TypeDefinition @"
        using System;
        using System.Runtime.InteropServices;
        public class RawPrinterHelper {{
            [DllImport("winspool.drv", CharSet=CharSet.Ansi)]
            public static extern IntPtr OpenPrinter(string pPrinterName, IntPtr pDefault, IntPtr pInputData);
            [DllImport("winspool.drv")]
            public static extern bool ClosePrinter(IntPtr hPrinter);
            [DllImport("winspool.drv", CharSet=CharSet.Ansi)]
            public static extern bool StartDocPrinter(IntPtr hPrinter, int level, ref DOCINFO di);
            [DllImport("winspool.drv")]
            public static extern bool EndDocPrinter(IntPtr hPrinter);
            [DllImport("winspool.drv")]
            public static extern bool WritePrinter(IntPtr hPrinter, IntPtr pBuf, int cbBuf, out int pcWritten);
            
            [StructLayout(LayoutKind.Sequential, CharSet=CharSet.Ansi)]
            public struct DOCINFO {{
                public string pDocName;
                public string pOutputFile;
                public string pDatatype;
            }}
        }}
"@
        
        $hPrinter = [RawPrinterHelper]::OpenPrinter($printerName, [IntPtr]::Zero, [IntPtr]::Zero)
        if ($hPrinter -ne [IntPtr]::Zero) {{
            $docInfo = New-Object RawPrinterHelper+DOCINFO
            $docInfo.pDocName = "Billiard"
            $docInfo.pOutputFile = $null
            $docInfo.pDatatype = "RAW"
            
            if ([RawPrinterHelper]::StartDocPrinter($hPrinter, 1, [ref]$docInfo)) {{
                $written = 0
                $ok = [RawPrinterHelper]::WritePrinter($hPrinter, $ptr, $bytes.Length, [ref]$written)
                [RawPrinterHelper]::EndDocPrinter($hPrinter) | Out-Null
                if ($ok -and $written -gt 0) {{ $success = $true }}
            }}
            [RawPrinterHelper]::ClosePrinter($hPrinter) | Out-Null
        }}
        
        [Runtime.InteropServices.Marshal]::FreeHGlobal($ptr)
    }}
}} catch {{
    $errorMsg = $_.Exception.Message
}}

Remove-Item "$tempFile" -Force -ErrorAction SilentlyContinue

if ($success) {{
    Write-Output "SUCCESS"
}} else {{
    Write-Output "ERROR: $errorMsg"
}}
"#, printer_name_escaped, temp_file_path);

    use std::process::Command;
    let output2 = Command::new("powershell")
        .args(["-NoProfile", "-Command", &ps_script])
        .output()
        .map_err(|e| format!("执行打印命令失败: {}", e))?;

    let stdout = String::from_utf8_lossy(&output2.stdout).trim().to_string();

    // 清理临时文件
    let _ = std::fs::remove_file(&temp_file);

    if stdout.contains("SUCCESS") {
        info!("Windows RawPrint SUCCESS");
        return Ok(());
    }

    Err(format!("Windows打印失败: {}", stdout))
}

/// 获取Windows打印机信息：名称和端口
#[allow(dead_code)]
#[cfg(windows)]
fn get_windows_printer_info() -> Result<(String, String), String> {
    use std::process::Command;

    let mut report: Vec<String> = Vec::new();

    // 方案1: 优先查找默认打印机，如果没有则获取第一个可用的真实打印机
    // 过滤掉虚拟打印机（Microsoft Print to PDF, XPS Document Viewer, Fax等）
    let ps_script_any = r#"
        $virtualPrinters = @('Microsoft Print to PDF', 'Microsoft XPS Document Viewer', 'Microsoft XPS Document Writer', 'Fax')
        $printers = @()
        try { $printers = Get-Printer } catch { $printers = @() }
        if ($printers.Count -eq 0) {
            try { $printers = Get-CimInstance Win32_Printer } catch { $printers = @() }
        }
        # 过滤掉虚拟打印机
        $realPrinters = $printers | Where-Object { $virtualPrinters -notcontains $_.Name }
        $default = $printers | Where-Object { $_.Default -eq $true -and $virtualPrinters -notcontains $_.Name } | Select-Object -First 1
        $first = $realPrinters | Select-Object -First 1
        if ($default) {
            Write-Output "default|$($default.Name)|$($default.PortName)"
        } elseif ($first) {
            Write-Output "first|$($first.Name)|$($first.PortName)"
        }
    "#;
    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", ps_script_any])
        .output();
    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let parts: Vec<&str> = stdout.split('|').collect();
        error!(
            "[agent-debug][H6] Get-Printer query status_success={} stdout='{}' stderr='{}' parts_len={}",
            output.status.success(),
            stdout,
            stderr,
            parts.len()
        );
        report.push(format!(
            "Get-Printer status_success={} stdout='{}' stderr='{}'",
            output.status.success(),
            stdout,
            stderr
        ));
        if parts.len() >= 3 {
            let marker = parts[0].trim();
            let printer_name = parts[1].trim();
            let port_name = parts[2].trim();
            if !port_name.is_empty() {
                if marker == "default" {
                    info!("Found default printer '{}' on port '{}'", printer_name, port_name);
                    return Ok((printer_name.to_string(), port_name.to_string()));
                } else {
                    info!("Found first available printer '{}' on port '{}' (no default set)", printer_name, port_name);
                    return Ok((printer_name.to_string(), port_name.to_string()));
                }
            }
        }
    } else if let Err(e) = output {
        error!("[agent-debug][H6] Get-Printer execution failed: {}", e);
        report.push(format!("Get-Printer exec_failed='{}'", e));
    }

    // 方案2: 获取所有真实打印机列表并尝试第一个
    let ps_script_all = r#"
        $virtualPrinters = @('Microsoft Print to PDF', 'Microsoft XPS Document Viewer', 'Microsoft XPS Document Writer', 'Fax')
        $printers = @()
        try { $printers = Get-Printer } catch { $printers = @() }
        if ($printers.Count -eq 0) {
            try { $printers = Get-CimInstance Win32_Printer } catch { $printers = @() }
        }
        $realPrinters = $printers | Where-Object { $virtualPrinters -notcontains $_.Name }
        if ($realPrinters.Count -gt 0) {
            $realPrinters | Select-Object -First 1 | ForEach-Object {
                Write-Output "$($_.Name)|$($_.PortName)"
            }
        }
    "#;
    let output2 = Command::new("powershell")
        .args(["-NoProfile", "-Command", ps_script_all])
        .output();
    if let Ok(output) = output2 {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let _ = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let parts: Vec<&str> = stdout.split('|').collect();
        if parts.len() >= 2 {
            let printer_name = parts[0].trim();
            let port_name = parts[1].trim();
            if !port_name.is_empty() {
                info!("Found first printer '{}' on port '{}' via list query", printer_name, port_name);
                return Ok((printer_name.to_string(), port_name.to_string()));
            }
        }
    }

    let one_line_report = report
        .into_iter()
        .map(|s| s.replace('\n', "\\n").replace('\r', "\\r"))
        .collect::<Vec<_>>()
        .join(" || ");
    Err(format!(
        "未找到可用的Windows打印机 [debug-report] {}",
        one_line_report
    ))
}

/// 通过USB发送打印数据 (非Windows)
#[cfg(not(windows))]
fn send_to_usb_printer(data: &[u8]) -> Result<(), String> {
    let context = rusb::GlobalContext::default();
    let devices = match context.devices() {
        Ok(d) => d,
        Err(e) => {
            return Err(format!("无法访问USB设备: {}\n\n请检查:\n1. 打印机是否已连接到电脑\n2. 是否安装了打印机驱动\n3. Windows: 以管理员身份运行程序\n4. macOS: 可能在系统偏好设置中授权访问", e));
        }
    };

    let mut last_error = String::new();
    let mut printer_count = 0;

    // 查找第一个可用的USB打印机
    for device in devices.iter() {
        let desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        let vid = desc.vendor_id();
        let pid = desc.product_id();

        // 检查是否在已知打印机列表中或为打印机类设备
        let is_known_printer = USB_PRINTER_DEVICES.iter().any(|&(v, p, _)| {
            (v == vid && p == pid) || (v == 0x0000 && p == 0x0000 && (v != 0 || p != 0))
        });
        let is_printer_class = desc.class_code() == 0x07;

        if is_known_printer || is_printer_class {
            printer_count += 1;
        }

        if !is_known_printer && !is_printer_class {
            continue;
        }

        // 尝试打开设备 (rusb 0.9 API: open() 不需要参数)
        match device.open() {
            Ok(mut handle) => {
                // 尝试发送数据
                match send_usb_data(&mut handle, data) {
                    Ok(_) => {
                        info!("USB打印成功: {:04x}:{:04x}", vid, pid);
                        return Ok(());
                    }
                    Err(e) => {
                        warn!("USB设备 {:04x}:{:04x} 打印失败: {}, 尝试下一个设备", vid, pid, e);
                        last_error = e;
                        continue;
                    }
                }
            }
            Err(e) => {
                // 设备被占用或权限不足，尝试下一个
                warn!("无法打开USB设备 {:04x}:{:04x}: {}", vid, pid, e);
                last_error = format!("无法打开设备: {}", e);
                continue;
            }
        }
    }

    if printer_count == 0 {
        Err("未发现USB打印机设备\n\n请检查:\n1. 打印机是否已通过USB连接到电脑\n2. 打印机是否已开机\n3. 尝试更换USB接口或数据线".to_string())
    } else if last_error.is_empty() {
        Err("未找到可用的USB打印机，请检查设备连接或权限\n\nWindows常见问题:\n1. 以管理员身份运行程序\n2. 在Windows设置中允许程序访问设备".to_string())
    } else {
        Err(format!("USB打印失败: {}", last_error))
    }
}

/// 向USB打印机发送数据 (Bulk Transfer)
#[allow(dead_code)]
fn send_usb_data<T: UsbContext>(
    handle: &mut DeviceHandle<T>,
    data: &[u8],
) -> Result<(), String> {
    // 查找合适的接口和端点
    let config = handle.active_configuration()
        .map_err(|e| format!("获取配置失败: {}", e))?;

    let device = handle.device();
    let config_desc = device.config_descriptor(config - 1)
        .map_err(|e| format!("获取配置描述符失败: {}", e))?;

    // 查找打印机接口 (通常为 class 7)
    for interface in config_desc.interfaces() {
        // 遍历接口的所有描述符（设置）
        for interface_desc in interface.descriptors() {
            // 打印机类接口 (class 7) 或 供应商特定接口 (class 255)
            if interface_desc.class_code() == 0x07 || interface_desc.class_code() == 0xFF {
                // 查找 Bulk OUT 端点
                for endpoint in interface_desc.endpoint_descriptors() {
                    if endpoint.direction() == rusb::Direction::Out
                        && endpoint.transfer_type() == rusb::TransferType::Bulk
                    {
                        let endpoint_addr = endpoint.address();
                        let interface_num = interface_desc.interface_number();

                        // Claim接口 (在某些系统上需要)
                        if let Err(e) = handle.claim_interface(interface_num) {
                            warn!("Claim接口失败: {}, 尝试继续", e);
                        }

                        // 执行 Bulk Transfer
                        let timeout = Duration::from_secs(10);
                        let bytes_written = handle.write_bulk(endpoint_addr, data, timeout)
                            .map_err(|e| format!("USB Bulk传输失败: {}", e))?;

                        info!("USB打印数据已发送: {} 字节", bytes_written);

                        // 释放接口
                        let _ = handle.release_interface(interface_num);

                        return Ok(());
                    }
                }
            }
        }
    }

    Err("未找到有效的打印机端点".to_string())
}

pub fn create_printer(req: CreatePrinterRequest) -> Result<Printer, String> {
    let conn = DB.lock();
    let paper_width = req.paper_width.unwrap_or(80);
    let port = match req.connection_type.as_str() {
        "network" => req.port.unwrap_or(9100),
        _ => req.port.unwrap_or(0),
    };
    let result = conn.execute(
        "INSERT INTO printers (name, connection_type, ip_address, port, serial_port, baud_rate, paper_width, is_enabled, is_default)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 1, 0)",
        params![req.name, req.connection_type, req.ip_address, port, req.serial_port, req.baud_rate, paper_width],
    );
    match result {
        Ok(_) => {
            let id = conn.last_insert_rowid();
            info!("Printer created: {} (id={})", req.name, id);
            drop(conn);
            get_printer_by_id(id)
        }
        Err(e) => Err(format!("创建打印机失败: {}", e)),
    }
}

pub fn update_printer(id: i64, req: CreatePrinterRequest) -> Result<Printer, String> {
    let conn = DB.lock();
    let paper_width = req.paper_width.unwrap_or(80);
    let port = match req.connection_type.as_str() {
        "network" => req.port.unwrap_or(9100),
        _ => req.port.unwrap_or(0),
    };
    let result = conn.execute(
        "UPDATE printers SET name=?1, connection_type=?2, ip_address=?3, port=?4, serial_port=?5, baud_rate=?6, paper_width=?7
         WHERE id=?8",
        params![req.name, req.connection_type, req.ip_address, port, req.serial_port, req.baud_rate, paper_width, id],
    );
    match result {
        Ok(_) => {
            info!("Printer updated: {} (id={})", req.name, id);
            drop(conn);
            get_printer_by_id(id)
        }
        Err(e) => Err(format!("更新打印机失败: {}", e)),
    }
}

pub fn delete_printer(id: i64) -> Result<(), String> {
    let conn = DB.lock();
    conn.execute("DELETE FROM printers WHERE id=?1", params![id])
        .map_err(|e| format!("删除打印机失败: {}", e))?;
    info!("Printer deleted: id={}", id);
    Ok(())
}

pub fn get_printers() -> Vec<Printer> {
    let conn = DB.lock();
    let mut stmt = match conn.prepare("SELECT id, name, connection_type, ip_address, port, serial_port, baud_rate, paper_width, is_enabled, is_default, created_at FROM printers ORDER BY is_default DESC, id ASC") {
        Ok(s) => s,
        Err(_) => return vec![],
    };
    let printers = stmt.query_map([], |row| {
        Ok(Printer {
            id: row.get(0)?,
            name: row.get(1)?,
            connection_type: row.get(2)?,
            ip_address: row.get(3)?,
            port: row.get(4)?,
            serial_port: row.get(5)?,
            baud_rate: row.get(6)?,
            paper_width: row.get(7)?,
            is_enabled: row.get::<_, i32>(8)? != 0,
            is_default: row.get::<_, i32>(9)? != 0,
            created_at: row.get(10)?,
        })
    });
    match printers {
        Ok(iter) => iter.filter_map(|r| r.ok()).collect(),
        Err(_) => vec![],
    }
}

fn get_printer_by_id(id: i64) -> Result<Printer, String> {
    let conn = DB.lock();
    conn.query_row(
        "SELECT id, name, connection_type, ip_address, port, serial_port, baud_rate, paper_width, is_enabled, is_default, created_at FROM printers WHERE id=?1",
        params![id],
        |row| {
            Ok(Printer {
                id: row.get(0)?,
                name: row.get(1)?,
                connection_type: row.get(2)?,
                ip_address: row.get(3)?,
                port: row.get(4)?,
                serial_port: row.get(5)?,
                baud_rate: row.get(6)?,
                paper_width: row.get(7)?,
                is_enabled: row.get::<_, i32>(8)? != 0,
                is_default: row.get::<_, i32>(9)? != 0,
                created_at: row.get(10)?,
            })
        },
    ).map_err(|e| format!("获取打印机失败: {}", e))
}

fn get_default_printer() -> Option<Printer> {
    let conn = DB.lock();
    conn.query_row(
        "SELECT id, name, connection_type, ip_address, port, serial_port, baud_rate, paper_width, is_enabled, is_default, created_at FROM printers WHERE is_default=1 AND is_enabled=1 LIMIT 1",
        [],
        |row| {
            Ok(Printer {
                id: row.get(0)?,
                name: row.get(1)?,
                connection_type: row.get(2)?,
                ip_address: row.get(3)?,
                port: row.get(4)?,
                serial_port: row.get(5)?,
                baud_rate: row.get(6)?,
                paper_width: row.get(7)?,
                is_enabled: row.get::<_, i32>(8)? != 0,
                is_default: row.get::<_, i32>(9)? != 0,
                created_at: row.get(10)?,
            })
        },
    ).ok()
}

pub fn set_default_printer(id: i64) -> Result<(), String> {
    let conn = DB.lock();
    conn.execute("UPDATE printers SET is_default=0", [])
        .map_err(|e| format!("清除默认打印机失败: {}", e))?;
    conn.execute("UPDATE printers SET is_default=1 WHERE id=?1", params![id])
        .map_err(|e| format!("设置默认打印机失败: {}", e))?;
    info!("Default printer set to id={}", id);
    Ok(())
}

pub fn toggle_printer(id: i64) -> Result<Printer, String> {
    let conn = DB.lock();
    conn.execute(
        "UPDATE printers SET is_enabled = NOT is_enabled WHERE id=?1",
        params![id],
    ).map_err(|e| format!("切换打印机状态失败: {}", e))?;
    drop(conn);
    get_printer_by_id(id)
}

pub fn print_receipt(req: PrintReceiptRequest) -> PrintResult {
    let printer = match req.printer_id {
        Some(id) => match get_printer_by_id(id) {
            Ok(p) => p,
            Err(e) => return PrintResult { success: false, message: e },
        },
        None => match get_default_printer() {
            Some(p) => p,
            None => return PrintResult { success: false, message: "未找到默认打印机，请先添加并设置默认打印机".to_string() },
        },
    };

    if !printer.is_enabled {
        return PrintResult { success: false, message: format!("打印机 '{}' 已禁用", printer.name) };
    }

    let data = generate_receipt_bytes(&req, printer.paper_width);

    let result = match printer.connection_type.as_str() {
        "network" => {
            match (&printer.ip_address, printer.port) {
                (Some(ip), Some(port)) => send_to_network_printer(ip, port, &data),
                _ => Err("网络打印机需要设置IP地址和端口".to_string()),
            }
        }
        "serial" => {
            match (&printer.serial_port, printer.baud_rate) {
                (Some(serial_port), Some(baud_rate)) => send_to_serial_printer(serial_port, baud_rate, &data),
                _ => Err("串口打印机需要设置串口号和波特率".to_string()),
            }
        }
        "usb" => {
            #[cfg(windows)]
            { print_receipt_bitmap(&req, &printer.name) }
            #[cfg(not(windows))]
            { send_to_usb_printer(&data) }
        }
        other => Err(format!("不支持的连接类型: {}", other)),
    };

    match result {
        Ok(()) => {
            info!("Receipt printed successfully via printer '{}'", printer.name);
            PrintResult { success: true, message: "打印成功".to_string() }
        }
        Err(e) => {
            error!("Print failed via printer '{}': {}", printer.name, e);
            PrintResult { success: false, message: e }
        }
    }
}

pub fn test_printer(id: i64) -> PrintResult {
    let printer = match get_printer_by_id(id) {
        Ok(p) => p,
        Err(e) => return PrintResult { success: false, message: e },
    };

    if !printer.is_enabled {
        return PrintResult { success: false, message: format!("打印机 '{}' 已禁用", printer.name) };
    }

    let mut buf = Vec::new();
    buf.extend(esc_pos_init());
    buf.extend(esc_pos_align_center());
    buf.extend(esc_pos_bold_on());
    buf.extend(esc_pos_text("打印机测试"));
    buf.extend(esc_pos_bold_off());
    buf.extend(esc_pos_divider(printer.paper_width));
    buf.extend(esc_pos_align_left());
    buf.extend(esc_pos_text(&format!("名称: {}", printer.name)));
    buf.extend(esc_pos_text(&format!("类型: {}", printer.connection_type)));
    if let Some(ref ip) = printer.ip_address {
        buf.extend(esc_pos_text(&format!("地址: {}:{}", ip, printer.port.unwrap_or(0))));
    }
    if let Some(ref sp) = printer.serial_port {
        buf.extend(esc_pos_text(&format!("串口: {} ({})", sp, printer.baud_rate.unwrap_or(0))));
    }
    buf.extend(esc_pos_text(&format!("纸宽: {}mm", printer.paper_width)));
    buf.extend(esc_pos_divider(printer.paper_width));

    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    buf.extend(esc_pos_align_center());
    buf.extend(esc_pos_text(&now));
    buf.extend(esc_pos_text("测试页打印成功！"));
    buf.extend(esc_pos_feed(3));
    buf.extend(esc_pos_cut());

    let result = match printer.connection_type.as_str() {
        "network" => {
            match (&printer.ip_address, printer.port) {
                (Some(ip), Some(port)) => send_to_network_printer(ip, port, &buf),
                _ => Err("网络打印机需要设置IP地址和端口".to_string()),
            }
        }
        "serial" => {
            match (&printer.serial_port, printer.baud_rate) {
                (Some(serial_port), Some(baud_rate)) => send_to_serial_printer(serial_port, baud_rate, &buf),
                _ => Err("串口打印机需要设置串口号和波特率".to_string()),
            }
        }
        "usb" => {
            #[cfg(windows)]
            {
                let test_req = PrintReceiptRequest {
                    printer_id: Some(printer.id),
                    shop_name: "打印机测试".to_string(),
                    order_no: Some("TEST-001".to_string()),
                    table_name: Some("1号桌".to_string()),
                    member_name: None,
                    start_time: Some("2025-01-01 12:00".to_string()),
                    end_time: Some("2025-01-01 14:00".to_string()),
                    duration_minutes: Some(120),
                    receipt_type: "normal".to_string(),
                    items: None,
                    total_amount: 100.0,
                    discount_amount: None,
                    deposit: None,
                    final_amount: 100.0,
                    payment_method: Some("现金".to_string()),
                };
                print_receipt_bitmap(&test_req, &printer.name)
            }
            #[cfg(not(windows))]
            send_to_usb_printer(&buf)
        }
        other => Err(format!("不支持的连接类型: {}", other)),
    };

    match result {
        Ok(()) => {
            info!("Test page printed successfully via printer '{}'", printer.name);
            PrintResult { success: true, message: "测试页打印成功".to_string() }
        }
        Err(e) => {
            error!("Test print failed via printer '{}': {}", printer.name, e);
            PrintResult { success: false, message: e }
        }
    }
}
