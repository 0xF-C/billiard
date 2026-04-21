#[cfg(windows)]
use crate::lib::models::PrintReceiptRequest;
#[cfg(windows)]
use ab_glyph::{FontRef, PxScale};
#[cfg(windows)]
use image::{GrayImage, Luma};
#[cfg(windows)]
use log::info;
#[cfg(windows)]
use once_cell::sync::Lazy;
#[cfg(windows)]
use std::sync::Mutex;

#[allow(dead_code)]
#[cfg(windows)]
static FONT: Lazy<Mutex<Option<(Vec<u8>, FontRef<'static>)>>> = Lazy::new(|| Mutex::new(None));

#[cfg(windows)]
const BITMAP_WIDTH: u32 = 384;

#[cfg(windows)]
fn init_font() -> Result<FontRef<'static>, String> {
    // Note: ab_glyph FontRef does not support .ttc (TrueType Collection), only .ttf
    let paths = [
        "C:\\Windows\\Fonts\\simhei.ttf",
        "C:\\Windows\\Fonts\\msyhbd.ttf",
        "C:\\Windows\\Fonts\\simkai.ttf",
        "C:\\Windows\\Fonts\\simfang.ttf",
    ];
    
    for path in &paths {
        if let Ok(data) = std::fs::read(path) {
            let static_data: &'static [u8] = Box::leak(data.into_boxed_slice());
            if let Ok(font) = FontRef::try_from_slice(static_data) {
                let mut lock = FONT.lock().unwrap();
                *lock = Some((static_data.to_vec(), font.clone()));
                return Ok(font);
            }
        }
    }
    
    Err("未找到中文字体，请安装黑体或楷体".to_string())
}
    
    Err("未找到中文字体(.ttf)。请确保系统安装了 simhei.ttf".to_string())
}

#[cfg(windows)]
fn get_font() -> Result<FontRef<'static>, String> {
    {
        let lock = FONT.lock().unwrap();
        if let Some((_, ref font)) = *lock {
            return Ok(font.clone());
        }
    }
    // If not loaded, try to load without blocking
    init_font()
}

#[cfg(windows)]
fn text_pixel_width(text: &str, font_size: f32) -> u32 {
    // 中文字符宽度 ≈ font_size，ASCII ≈ font_size * 0.6
    let width: f32 = text.chars().map(|c| {
        if c.is_ascii() { font_size * 0.6 } else { font_size }
    }).sum();
    width as u32
}

#[cfg(windows)]
fn render_receipt_bitmap(req: &PrintReceiptRequest) -> Result<GrayImage, String> {
    let width = BITMAP_WIDTH;
    let font_size = 22.0;
    let scale = PxScale::from(font_size);
    let line_height = 34u32;
    let margin = 10u32;
    
    let font = get_font()?;
    info!("Got font, ready to render bitmap");
    
    struct Line {
        text: String,
        x: u32,
        is_divider: bool,
        align_center: bool,
    }
    
    let mut lines: Vec<Line> = Vec::new();
    
    lines.push(Line { text: req.shop_name.clone(), x: 0, is_divider: false, align_center: true });
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    lines.push(Line { text: now, x: 0, is_divider: false, align_center: true });
    lines.push(Line { text: String::new(), x: 0, is_divider: true, align_center: false });
    
    if let Some(ref v) = req.order_no {
        lines.push(Line { text: format!("订单号: {}", v), x: margin, is_divider: false, align_center: false });
    }
    if let Some(ref v) = req.table_name {
        lines.push(Line { text: format!("球桌: {}", v), x: margin, is_divider: false, align_center: false });
    }
    if let Some(ref v) = req.member_name {
        lines.push(Line { text: format!("会员: {}", v), x: margin, is_divider: false, align_center: false });
    }
    if let Some(ref v) = req.start_time {
        lines.push(Line { text: format!("开始时间: {}", v), x: margin, is_divider: false, align_center: false });
    }
    if let Some(ref v) = req.end_time {
        lines.push(Line { text: format!("结束时间: {}", v), x: margin, is_divider: false, align_center: false });
    }
    if let Some(v) = req.duration_minutes {
        lines.push(Line { text: format!("时长: {}分钟", v), x: margin, is_divider: false, align_center: false });
    }
    
    lines.push(Line { text: String::new(), x: 0, is_divider: true, align_center: false });
    
    if req.receipt_type == "sale" {
        if let Some(ref items) = req.items {
            for item in items {
                lines.push(Line { 
                    text: format!("{} x{}  ¥{:.2}", item.name, item.quantity, item.price),
                    x: margin, is_divider: false, align_center: false 
                });
            }
            lines.push(Line { text: String::new(), x: 0, is_divider: true, align_center: false });
        }
    }
    
    lines.push(Line { text: format!("合计: ¥{:.2}", req.total_amount), x: margin, is_divider: false, align_center: false });
    
    if let Some(discount) = req.discount_amount {
        if discount > 0.0 {
            lines.push(Line { text: format!("优惠: -¥{:.2}", discount), x: margin, is_divider: false, align_center: false });
        }
    }
    if let Some(deposit) = req.deposit {
        if deposit > 0.0 {
            lines.push(Line { text: format!("押金: -¥{:.2}", deposit), x: margin, is_divider: false, align_center: false });
        }
    }
    
    lines.push(Line { text: String::new(), x: 0, is_divider: true, align_center: false });
    lines.push(Line { text: format!("实收: ¥{:.2}", req.final_amount), x: margin, is_divider: false, align_center: false });
    
    if let Some(ref method) = req.payment_method {
        lines.push(Line { text: format!("支付方式: {}", method), x: margin, is_divider: false, align_center: false });
    }
    
    lines.push(Line { text: String::new(), x: 0, is_divider: true, align_center: false });
    lines.push(Line { text: "感谢您的光临！".to_string(), x: 0, is_divider: false, align_center: true });
    lines.push(Line { text: "欢迎下次再来".to_string(), x: 0, is_divider: false, align_center: true });
    
    let height = lines.len() as u32 * line_height + margin * 2;
    let mut img = GrayImage::from_pixel(width, height, Luma([255u8]));
    
    let mut y = margin;
    for line in &lines {
        if line.is_divider {
            let yy = y + line_height / 2;
            for x in 0..width {
                img.put_pixel(x, yy, Luma([0u8]));
            }
        } else {
            let x = if line.align_center {
                let tw = text_pixel_width(&line.text, font_size);
                (width.saturating_sub(tw)) / 2
            } else {
                line.x
            };
            
            let color = Luma([0u8]);
            imageproc::drawing::draw_text_mut(&mut img, color, x as i32, y as i32, scale, &font, &line.text);
        }
        y += line_height;
    }
    
    Ok(img)
}

#[cfg(windows)]
fn bitmap_to_esc_pos(img: &GrayImage) -> Vec<u8> {
    let width = img.width() as u16;
    let height = img.height() as u16;
    let row_bytes = (width + 7) / 8;
    
    let esc = 0x1B as u8;
    let gs = 0x1D as u8;
    let mut data = vec![esc, b'@'];
    
    data.push(gs);
    data.push(b'v');
    data.push(b'0');
    data.push(0x00);
    // xL/xH = 每行字节数（不是像素宽度！）
    data.push((row_bytes & 0xFF) as u8);
    data.push((row_bytes >> 8) as u8);
    data.push((height & 0xFF) as u8);
    data.push((height >> 8) as u8);
    
    for y in 0..img.height() {
        let mut row_data = vec![0u8; row_bytes as usize];
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y)[0];
            if pixel < 128 {
                let byte_idx = (x / 8) as usize;
                let bit_idx = 7 - (x % 8);
                row_data[byte_idx] |= 1 << bit_idx;
            }
        }
        data.extend_from_slice(&row_data);
    }
    
    data.extend_from_slice(&[gs, b'V', 0x00]);
    
    data
}

#[cfg(windows)]
pub fn print_receipt_bitmap(req: &PrintReceiptRequest, printer_name: &str) -> Result<(), String> {
    info!("Generating receipt bitmap for printer '{}'...", printer_name);
    let bitmap = render_receipt_bitmap(req)?;
    info!("Bitmap size: {}x{}", bitmap.width(), bitmap.height());
    
    let data = bitmap_to_esc_pos(&bitmap);
    info!("ESC/POS bitmap data: {} bytes", data.len());
    
    use std::os::windows::ffi::OsStrExt;
    let printer_name_wide: Vec<u16> = std::ffi::OsStr::new(printer_name)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let mut printer_name_buf = printer_name_wide;
    let printer_name_ptr = printer_name_buf.as_mut_ptr();
    
    use winapi::shared::minwindef::{BOOL, DWORD, LPVOID};
    use winapi::shared::ntdef::HANDLE;
    use winapi::um::winspool::{
        ClosePrinter, EndDocPrinter, EndPagePrinter, OpenPrinterW, StartDocPrinterW,
        StartPagePrinter, WritePrinter, DOC_INFO_1W,
    };
    
    unsafe {
        let mut hprinter: HANDLE = std::ptr::null_mut();
        let ok: BOOL = OpenPrinterW(printer_name_ptr, &mut hprinter, std::ptr::null_mut());
        if ok == 0 {
            return Err(format!("OpenPrinter 失败: '{}'", printer_name));
        }
        
        let doc_name: Vec<u16> = std::ffi::OsStr::new("Receipt")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let data_type: Vec<u16> = std::ffi::OsStr::new("RAW")
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        
        let mut doc_name_buf = doc_name;
        let mut data_type_buf = data_type;
        
        let doc_info = DOC_INFO_1W {
            pDocName: doc_name_buf.as_mut_ptr(),
            pOutputFile: std::ptr::null_mut(),
            pDatatype: data_type_buf.as_mut_ptr(),
        };
        
        let job_id: DWORD = StartDocPrinterW(hprinter, 1, std::ptr::addr_of!(doc_info) as *mut u8);
        if job_id == 0 {
            ClosePrinter(hprinter);
            return Err("StartDocPrinter 失败".to_string());
        }
        
        StartPagePrinter(hprinter);
        
        let mut written: DWORD = 0;
        let ok: BOOL = WritePrinter(
            hprinter,
            data.as_ptr() as LPVOID,
            data.len() as DWORD,
            &mut written,
        );
        
        EndPagePrinter(hprinter);
        EndDocPrinter(hprinter);
        ClosePrinter(hprinter);
        
        if ok == 0 || (written as usize) != data.len() {
            return Err(format!("WritePrinter 失败: {}/{} bytes", written, data.len()));
        }
    }
    
    info!("Bitmap receipt printed successfully");
    Ok(())
}