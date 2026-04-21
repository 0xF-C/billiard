use crate::lib::models::PrintReceiptRequest;
use crate::lib::services::printer::ESC;
use log::info;

#[cfg(windows)]
use ab_glyph::{FontRef, Font, PxScale};
#[cfg(windows)]
use image::{GrayImage, Luma, ImageBuffer};
#[cfg(windows)]
use imageproc::drawing::draw_text_mut;

#[cfg(windows)]
const BITMAP_WIDTH: u32 = 384;

#[cfg(windows)]
fn load_font() -> Result<FontRef<'static>, String> {
    let paths = [
        "C:\\Windows\\Fonts\\simhei.ttf",
        "C:\\Windows\\Fonts\\msyh.ttc",
        "C:\\Windows\\Fonts\\simsun.ttc",
        "C:\\Windows\\Fonts\\msyhbd.ttf",
    ];
    
    for path in &paths {
        if let Ok(data) = std::fs::read(path) {
            if let Ok(font) = FontRef::try_from_slice(&data) {
                info!("Loaded font from {}", path);
                return Ok(font);
            }
        }
    }
    
    Err("未找到中文字体。请确保系统安装了 simhei.ttf 或 msyh.ttc。".to_string())
}

#[cfg(windows)]
fn render_receipt_bitmap(req: &PrintReceiptRequest) -> Result<GrayImage, String> {
    let width = BITMAP_WIDTH;
    let font_size = 22.0;
    let scale = PxScale::from(font_size);
    let line_height = 34u32;
    let margin = 10u32;
    
    let font = load_font()?;
    
    #[derive(Clone)]
    struct Line {
        text: String,
        x: u32,
        bold: bool,
        is_divider: bool,
        align_center: bool,
    }
    
    let mut lines: Vec<Line> = Vec::new();
    
    lines.push(Line { text: req.shop_name.clone(), x: 0, bold: true, is_divider: false, align_center: true });
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    lines.push(Line { text: now, x: 0, bold: false, is_divider: false, align_center: true });
    lines.push(Line { text: String::new(), x: 0, bold: false, is_divider: true, align_center: false });
    
    let mut add_line = |label: &str, value: &str| {
        lines.push(Line { text: format!("{} {}", label, value), x: margin, bold: false, is_divider: false, align_center: false });
    };
    
    if let Some(ref v) = req.order_no { add_line("订单号:", v); }
    if let Some(ref v) = req.table_name { add_line("球桌:", v); }
    if let Some(ref v) = req.member_name { add_line("会员:", v); }
    if let Some(ref v) = req.start_time { add_line("开始时间:", v); }
    if let Some(ref v) = req.end_time { add_line("结束时间:", v); }
    if let Some(v) = req.duration_minutes { add_line("时长:", &format!("{}分钟", v)); }
    
    lines.push(Line { text: String::new(), x: 0, bold: false, is_divider: true, align_center: false });
    
    if req.receipt_type == "sale" {
        if let Some(ref items) = req.items {
            for item in items {
                lines.push(Line { 
                    text: format!("{} x{}  ¥{:.2}", item.name, item.quantity, item.price),
                    x: margin, bold: false, is_divider: false, align_center: false 
                });
            }
            lines.push(Line { text: String::new(), x: 0, bold: false, is_divider: true, align_center: false });
        }
    }
    
    add_line("合计:", &format!("¥{:.2}", req.total_amount));
    
    if let Some(discount) = req.discount_amount {
        if discount > 0.0 {
            add_line("优惠:", &format!("-¥{:.2}", discount));
        }
    }
    if let Some(deposit) = req.deposit {
        if deposit > 0.0 {
            add_line("押金:", &format!("-¥{:.2}", deposit));
        }
    }
    
    lines.push(Line { text: String::new(), x: 0, bold: false, is_divider: true, align_center: false });
    add_line("实收:", &format!("¥{:.2}", req.final_amount));
    
    if let Some(ref method) = req.payment_method {
        add_line("支付方式:", method);
    }
    
    lines.push(Line { text: String::new(), x: 0, bold: false, is_divider: true, align_center: false });
    lines.push(Line { text: "感谢您的光临！".to_string(), x: 0, bold: false, is_divider: false, align_center: true });
    lines.push(Line { text: "欢迎下次再来".to_string(), x: 0, bold: false, is_divider: false, align_center: true });
    
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
                let tw = ab_glyph::pt_to_px(line.text.chars().count() as f32 * font_size * 0.6, font_size) as u32;
                ((width.saturating_sub(tw)) / 2)
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
    
    let mut data = vec![ESC, b'@'];
    
    let gs = 0x1D as u8;
    data.push(gs);
    data.push(b'v');
    data.push(b'0');
    data.push(0x00);
    data.push((width & 0xFF) as u8);
    data.push((width >> 8) as u8);
    data.push((height & 0xFF) as u8);
    data.push((height >> 8) as u8);
    
    for y in 0..img.height() {
        let mut byte = 0u8;
        let mut bit = 7i8;
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y)[0];
            if pixel < 128 {
                byte |= 1 << bit;
            }
            bit -= 1;
            if bit < 0 {
                data.push(byte);
                byte = 0;
                bit = 7;
            }
        }
        if bit != 7 {
            data.push(byte);
        }
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
    
    let printer_name_wide: Vec<u16> = std::ffi::OsStr::new(printer_name)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    
    use winapi::shared::minwindef::{BOOL, DWORD, LPVOID};
    use winapi::shared::ntdef::HANDLE;
    use winapi::um::winspool::{
        ClosePrinter, EndDocPrinter, EndPagePrinter, OpenPrinterW, StartDocPrinterW,
        StartPagePrinter, WritePrinter, DOC_INFO_1W,
    };
    
    unsafe {
        let mut hprinter: HANDLE = std::ptr::null_mut();
        let ok: BOOL = OpenPrinterW(printer_name_wide.as_ptr(), &mut hprinter, std::ptr::null_mut());
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
        
        let doc_info = DOC_INFO_1W {
            pDocName: doc_name.as_ptr(),
            pOutputFile: std::ptr::null(),
            pDatatype: data_type.as_ptr(),
        };
        
        let job_id: DWORD = StartDocPrinterW(hprinter, 1, &doc_info as *const _ as *mut _);
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