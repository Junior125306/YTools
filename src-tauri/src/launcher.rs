//! 用指定程序打开工作区目录，并读取 exe 的名称与图标

use serde::Serialize;
use std::path::Path;

/// 设置页展示、搜索窗口启动用的程序信息
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LauncherInfo {
    pub name: String,
    pub exe_path: String,
    pub icon: String,
}

/// 读取程序名称、路径和图标。图标失败时仍返回程序信息，icon 为空字符串。
#[tauri::command]
pub fn inspect_launcher(path: String) -> Result<LauncherInfo, String> {
    validate_program(&path)?;
    let program = Path::new(&path);
    let name = read_product_name(program).unwrap_or_else(|| file_stem_name(program));
    let icon = read_icon_data_url(program).unwrap_or_default();
    Ok(LauncherInfo {
        name,
        exe_path: path,
        icon,
    })
}

/// 用 exe 打开目录。目录作为第一个参数传给程序。
#[tauri::command]
pub fn open_with_launcher(exe_path: String, folder_path: String) -> Result<(), String> {
    validate_program(&exe_path)?;
    crate::system::validate_existing_directory(&folder_path)?;
    if !Path::new(&folder_path).is_dir() {
        return Err(format!("不是目录: {folder_path}"));
    }

    std::process::Command::new(&exe_path)
        .arg(&folder_path)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|err| format!("启动失败: {err}"))?;
    Ok(())
}

fn validate_program(path: &str) -> Result<(), String> {
    let program = Path::new(path);
    if crate::system::has_parent_dir(program) {
        return Err("不允许的程序路径".to_string());
    }
    if !program.is_file() {
        return Err(format!("程序不存在: {path}"));
    }
    #[cfg(windows)]
    {
        let extension = program
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        if !extension.eq_ignore_ascii_case("exe") {
            return Err("只能选择 exe 程序".to_string());
        }
    }
    Ok(())
}

fn file_stem_name(path: &Path) -> String {
    path.file_stem()
        .map(|stem| stem.to_string_lossy().trim().to_string())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| "程序".to_string())
}

#[cfg(not(windows))]
fn read_product_name(_path: &Path) -> Option<String> {
    None
}

#[cfg(not(windows))]
fn read_icon_data_url(_path: &Path) -> Result<String, String> {
    Err("当前系统不支持读取图标".to_string())
}

#[cfg(any(windows, test))]
fn unpremultiply(channel: u8, alpha: u8) -> u8 {
    if alpha == 0 {
        0
    } else {
        let value = (u16::from(channel) * 255) / u16::from(alpha);
        value.min(255) as u8
    }
}

/// 把绘制结果从 BGRA 转成 RGBA。
/// 全透明说明颜色位图没写 alpha，这时保留颜色并改为不透明。
/// 其余像素按预乘 alpha 还原。
#[cfg(any(windows, test))]
fn convert_bgra(src: &[u8]) -> Vec<u8> {
    let fully_transparent = src.chunks_exact(4).all(|pixel| pixel[3] == 0);
    let mut rgba = Vec::with_capacity(src.len());
    for pixel in src.chunks_exact(4) {
        let (blue, green, red, alpha) = (pixel[0], pixel[1], pixel[2], pixel[3]);
        if fully_transparent {
            rgba.extend_from_slice(&[red, green, blue, 255]);
        } else if alpha == 0 {
            rgba.extend_from_slice(&[0, 0, 0, 0]);
        } else {
            rgba.extend_from_slice(&[
                unpremultiply(red, alpha),
                unpremultiply(green, alpha),
                unpremultiply(blue, alpha),
                alpha,
            ]);
        }
    }
    rgba
}

/// AND 掩码：1 表示透明。mask 是自下而上的 1bpp 位图。
#[cfg(any(windows, test))]
fn apply_and_mask(bgra: &mut [u8], mask: &[u8], width: u32, height: u32) {
    let stride = ((width + 31) / 32) * 4;
    for y in 0..height {
        let mask_y = height - 1 - y;
        for x in 0..width {
            let byte_index = (mask_y * stride + x / 8) as usize;
            let transparent = if byte_index < mask.len() {
                let bit = 7 - (x % 8);
                (mask[byte_index] >> bit) & 1 == 1
            } else {
                false
            };
            let pixel = ((y * width + x) * 4) as usize;
            if pixel + 3 < bgra.len() {
                bgra[pixel + 3] = if transparent { 0 } else { 255 };
            }
        }
    }
}

#[cfg(any(windows, test))]
fn encode_png(width: u32, height: u32, rgba: &[u8]) -> Result<Vec<u8>, String> {
    let expected = (width as usize)
        .checked_mul(height as usize)
        .and_then(|count| count.checked_mul(4))
        .ok_or_else(|| "图标数据无效".to_string())?;
    if rgba.len() != expected || width == 0 || height == 0 {
        return Err("图标数据无效".to_string());
    }

    let mut output = Vec::new();
    let mut encoder = png::Encoder::new(&mut output, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder
        .write_header()
        .map_err(|err| format!("生成图标失败: {err}"))?;
    writer
        .write_image_data(rgba)
        .map_err(|err| format!("生成图标失败: {err}"))?;
    drop(writer);
    Ok(output)
}

#[cfg(windows)]
fn base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut output = String::new();
    let mut index = 0;
    while index + 3 <= data.len() {
        let chunk = (u32::from(data[index]) << 16)
            | (u32::from(data[index + 1]) << 8)
            | u32::from(data[index + 2]);
        output.push(TABLE[((chunk >> 18) & 63) as usize] as char);
        output.push(TABLE[((chunk >> 12) & 63) as usize] as char);
        output.push(TABLE[((chunk >> 6) & 63) as usize] as char);
        output.push(TABLE[(chunk & 63) as usize] as char);
        index += 3;
    }
    if index < data.len() {
        let remaining = data.len() - index;
        let mut chunk = u32::from(data[index]) << 16;
        if remaining == 2 {
            chunk |= u32::from(data[index + 1]) << 8;
        }
        output.push(TABLE[((chunk >> 18) & 63) as usize] as char);
        output.push(TABLE[((chunk >> 12) & 63) as usize] as char);
        if remaining == 2 {
            output.push(TABLE[((chunk >> 6) & 63) as usize] as char);
            output.push('=');
        } else {
            output.push('=');
            output.push('=');
        }
    }
    output
}

#[cfg(windows)]
fn to_wide(value: &std::ffi::OsStr) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    value.encode_wide().chain(std::iter::once(0)).collect()
}

#[cfg(windows)]
fn read_product_name(path: &Path) -> Option<String> {
    let wide = to_wide(path.as_os_str());
    unsafe {
        let mut handle = 0u32;
        let size = GetFileVersionInfoSizeW(wide.as_ptr(), &mut handle);
        if size == 0 || size > 1_048_576 {
            return None;
        }
        let mut block = vec![0u8; size as usize];
        if GetFileVersionInfoW(wide.as_ptr(), 0, size, block.as_mut_ptr()) == 0 {
            return None;
        }
        for (language, codepage) in version_translations(&block) {
            let queries = [
                format!("\\StringFileInfo\\{language:04X}{codepage:04X}\\ProductName"),
                format!("\\StringFileInfo\\{language:04x}{codepage:04x}\\ProductName"),
            ];
            for query in queries {
                if let Some(name) = query_version_string(&block, &query) {
                    return Some(name);
                }
            }
        }
        None
    }
}

#[cfg(windows)]
fn version_translations(block: &[u8]) -> Vec<(u16, u16)> {
    let query = to_wide(std::ffi::OsStr::new("\\VarFileInfo\\Translation"));
    let mut pointer = std::ptr::null_mut();
    let mut length = 0u32;
    let ok = unsafe { VerQueryValueW(block.as_ptr(), query.as_ptr(), &mut pointer, &mut length) };
    if ok == 0 || pointer.is_null() || length < 4 {
        return Vec::new();
    }
    let bytes = unsafe { bytes_inside(block, pointer, length as usize) };
    let mut pairs = Vec::new();
    let mut offset = 0;
    while offset + 4 <= bytes.len() {
        let language = u16::from_le_bytes([bytes[offset], bytes[offset + 1]]);
        let codepage = u16::from_le_bytes([bytes[offset + 2], bytes[offset + 3]]);
        pairs.push((language, codepage));
        offset += 4;
    }
    pairs
}

#[cfg(windows)]
fn query_version_string(block: &[u8], sub_block: &str) -> Option<String> {
    let query = to_wide(std::ffi::OsStr::new(sub_block));
    let mut pointer = std::ptr::null_mut();
    let mut length = 0u32;
    let ok = unsafe { VerQueryValueW(block.as_ptr(), query.as_ptr(), &mut pointer, &mut length) };
    if ok == 0 || pointer.is_null() {
        return None;
    }
    let bytes = unsafe { bytes_inside(block, pointer, block.len()) };
    let mut units = Vec::new();
    let mut index = 0;
    while index + 1 < bytes.len() && units.len() < 512 {
        let unit = u16::from_le_bytes([bytes[index], bytes[index + 1]]);
        if unit == 0 {
            break;
        }
        units.push(unit);
        index += 2;
    }
    let name = String::from_utf16_lossy(&units).trim().to_string();
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

#[cfg(windows)]
unsafe fn bytes_inside(block: &[u8], pointer: *mut std::ffi::c_void, max_len: usize) -> Vec<u8> {
    let start = pointer as usize;
    let block_start = block.as_ptr() as usize;
    let block_end = block_start + block.len();
    if start < block_start || start >= block_end {
        return Vec::new();
    }
    let available = block_end - start;
    let len = max_len.min(available);
    std::slice::from_raw_parts(pointer as *const u8, len).to_vec()
}

#[cfg(windows)]
fn read_icon_data_url(path: &Path) -> Result<String, String> {
    let _com = ComGuard::init();
    let png = exe_icon_png(path)?;
    Ok(format!("data:image/png;base64,{}", base64_encode(&png)))
}

#[cfg(windows)]
fn exe_icon_png(path: &Path) -> Result<Vec<u8>, String> {
    let wide = to_wide(path.as_os_str());
    let mut file_info: ShFileInfoW = unsafe { std::mem::zeroed() };
    let found = unsafe {
        SHGetFileInfoW(
            wide.as_ptr(),
            0,
            &mut file_info,
            std::mem::size_of::<ShFileInfoW>() as u32,
            SHGFI_ICON,
        )
    };
    if found == 0 || file_info.h_icon == 0 {
        return Err("读取图标失败".to_string());
    }

    let mut guard = GdiGuard {
        icon: file_info.h_icon,
        mask: 0,
        color: 0,
        screen_dc: 0,
        mem_dc: 0,
        dib: 0,
        old_obj: 0,
    };

    let mut icon_info: IconInfo = unsafe { std::mem::zeroed() };
    if unsafe { GetIconInfo(guard.icon, &mut icon_info) } != 0 {
        guard.mask = icon_info.hbm_mask;
        guard.color = icon_info.hbm_color;
    }

    let (width, height) = icon_size(&guard);
    if !(1..=256).contains(&width) || !(1..=256).contains(&height) {
        return Err("图标尺寸无效".to_string());
    }

    guard.screen_dc = unsafe { GetDC(0) };
    if guard.screen_dc == 0 {
        return Err("读取图标失败".to_string());
    }

    let mut header: BitmapInfoHeader = unsafe { std::mem::zeroed() };
    header.bi_size = std::mem::size_of::<BitmapInfoHeader>() as u32;
    header.bi_width = width as i32;
    header.bi_height = -(height as i32);
    header.bi_planes = 1;
    header.bi_bit_count = 32;

    let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
    guard.dib =
        unsafe { CreateDIBSection(guard.screen_dc, &header, DIB_RGB_COLORS, &mut bits, 0, 0) };
    if guard.dib == 0 || bits.is_null() {
        return Err("读取图标失败".to_string());
    }

    let byte_len = (width as usize)
        .saturating_mul(height as usize)
        .saturating_mul(4);
    unsafe { std::ptr::write_bytes(bits as *mut u8, 0, byte_len) };

    guard.mem_dc = unsafe { CreateCompatibleDC(guard.screen_dc) };
    if guard.mem_dc == 0 {
        return Err("读取图标失败".to_string());
    }
    guard.old_obj = unsafe { SelectObject(guard.mem_dc, guard.dib) };
    let drawn = unsafe {
        DrawIconEx(
            guard.mem_dc,
            0,
            0,
            guard.icon,
            width as i32,
            height as i32,
            0,
            0,
            DI_NORMAL,
        )
    };
    if drawn == 0 {
        return Err("读取图标失败".to_string());
    }

    let mut bgra = vec![0u8; byte_len];
    unsafe {
        std::ptr::copy_nonoverlapping(bits as *const u8, bgra.as_mut_ptr(), byte_len);
    }
    if bgra.chunks_exact(4).all(|pixel| pixel[3] == 0) {
        if let Some(mask) = read_and_mask(guard.screen_dc, guard.mask, width, height) {
            apply_and_mask(&mut bgra, &mask, width, height);
        }
    }
    let rgba = convert_bgra(&bgra);
    encode_png(width, height, &rgba)
}

#[cfg(windows)]
fn icon_size(guard: &GdiGuard) -> (u32, u32) {
    if guard.color != 0 {
        let mut bitmap: Bitmap = unsafe { std::mem::zeroed() };
        let read = unsafe {
            GetObjectW(
                guard.color,
                std::mem::size_of::<Bitmap>() as i32,
                &mut bitmap as *mut Bitmap as *mut std::ffi::c_void,
            )
        };
        if read > 0 && bitmap.bm_width > 0 && bitmap.bm_height != 0 {
            return (
                bitmap.bm_width.unsigned_abs(),
                bitmap.bm_height.unsigned_abs(),
            );
        }
    }
    let width = unsafe { GetSystemMetrics(SM_CXICON) }.max(32) as u32;
    let height = unsafe { GetSystemMetrics(SM_CYICON) }.max(32) as u32;
    (width.min(256), height.min(256))
}

#[cfg(windows)]
fn read_and_mask(dc: isize, mask: isize, width: u32, height: u32) -> Option<Vec<u8>> {
    if dc == 0 || mask == 0 {
        return None;
    }
    let mut info = MaskBitmapInfo {
        header: unsafe { std::mem::zeroed() },
        colors: [
            RgbQuad {
                blue: 0,
                green: 0,
                red: 0,
                reserved: 0,
            },
            RgbQuad {
                blue: 255,
                green: 255,
                red: 255,
                reserved: 0,
            },
        ],
    };
    info.header.bi_size = std::mem::size_of::<BitmapInfoHeader>() as u32;
    info.header.bi_width = width as i32;
    info.header.bi_height = height as i32;
    info.header.bi_planes = 1;
    info.header.bi_bit_count = 1;
    let stride = ((width + 31) / 32) * 4;
    let mut bits = vec![0u8; (stride as usize).saturating_mul(height as usize)];
    let lines = unsafe {
        GetDIBits(
            dc,
            mask,
            0,
            height,
            bits.as_mut_ptr() as *mut std::ffi::c_void,
            &mut info as *mut MaskBitmapInfo as *mut BitmapInfoHeader,
            DIB_RGB_COLORS,
        )
    };
    if lines == 0 {
        None
    } else {
        Some(bits)
    }
}

#[cfg(windows)]
const SHGFI_ICON: u32 = 0x0000_0100;
#[cfg(windows)]
const DI_NORMAL: u32 = 0x0003;
#[cfg(windows)]
const DIB_RGB_COLORS: u32 = 0;
#[cfg(windows)]
const SM_CXICON: i32 = 11;
#[cfg(windows)]
const SM_CYICON: i32 = 12;

#[cfg(windows)]
const _: () = assert!(std::mem::size_of::<BitmapInfoHeader>() == 40);

#[cfg(windows)]
#[repr(C)]
struct ShFileInfoW {
    h_icon: isize,
    i_icon: i32,
    dw_attributes: u32,
    sz_display_name: [u16; 260],
    sz_type_name: [u16; 80],
}

#[cfg(windows)]
#[repr(C)]
struct IconInfo {
    f_icon: i32,
    x_hotspot: u32,
    y_hotspot: u32,
    hbm_mask: isize,
    hbm_color: isize,
}

#[cfg(windows)]
#[repr(C)]
struct Bitmap {
    bm_type: i32,
    bm_width: i32,
    bm_height: i32,
    bm_width_bytes: i32,
    bm_planes: u16,
    bm_bits_pixel: u16,
    bm_bits: *mut std::ffi::c_void,
}

#[cfg(windows)]
#[repr(C)]
struct BitmapInfoHeader {
    bi_size: u32,
    bi_width: i32,
    bi_height: i32,
    bi_planes: u16,
    bi_bit_count: u16,
    bi_compression: u32,
    bi_size_image: u32,
    bi_x_pels_per_meter: i32,
    bi_y_pels_per_meter: i32,
    bi_clr_used: u32,
    bi_clr_important: u32,
}

#[cfg(windows)]
#[repr(C)]
struct RgbQuad {
    blue: u8,
    green: u8,
    red: u8,
    reserved: u8,
}

#[cfg(windows)]
#[repr(C)]
struct MaskBitmapInfo {
    header: BitmapInfoHeader,
    colors: [RgbQuad; 2],
}

#[cfg(windows)]
struct GdiGuard {
    icon: isize,
    mask: isize,
    color: isize,
    screen_dc: isize,
    mem_dc: isize,
    dib: isize,
    old_obj: isize,
}

#[cfg(windows)]
impl Drop for GdiGuard {
    fn drop(&mut self) {
        unsafe {
            if self.mem_dc != 0 && self.old_obj != 0 {
                SelectObject(self.mem_dc, self.old_obj);
                self.old_obj = 0;
            }
            if self.dib != 0 {
                DeleteObject(self.dib);
                self.dib = 0;
            }
            if self.mem_dc != 0 {
                DeleteDC(self.mem_dc);
                self.mem_dc = 0;
            }
            if self.screen_dc != 0 {
                ReleaseDC(0, self.screen_dc);
                self.screen_dc = 0;
            }
            if self.color != 0 {
                DeleteObject(self.color);
                self.color = 0;
            }
            if self.mask != 0 {
                DeleteObject(self.mask);
                self.mask = 0;
            }
            if self.icon != 0 {
                DestroyIcon(self.icon);
                self.icon = 0;
            }
        }
    }
}

#[cfg(windows)]
struct ComGuard(bool);

#[cfg(windows)]
impl ComGuard {
    fn init() -> Self {
        let hr = unsafe { CoInitializeEx(std::ptr::null_mut(), 0) };
        // S_OK = 0，S_FALSE = 1。这两种都增加了计数，结束时要配对释放。
        Self(hr == 0 || hr == 1)
    }
}

#[cfg(windows)]
impl Drop for ComGuard {
    fn drop(&mut self) {
        if self.0 {
            unsafe { CoUninitialize() }
        }
    }
}

#[cfg(windows)]
#[link(name = "shell32")]
extern "system" {
    fn SHGetFileInfoW(
        path: *const u16,
        attributes: u32,
        info: *mut ShFileInfoW,
        info_size: u32,
        flags: u32,
    ) -> usize;
}

#[cfg(windows)]
#[link(name = "user32")]
extern "system" {
    fn GetIconInfo(icon: isize, info: *mut IconInfo) -> i32;
    fn DestroyIcon(icon: isize) -> i32;
    fn DrawIconEx(
        hdc: isize,
        x: i32,
        y: i32,
        icon: isize,
        width: i32,
        height: i32,
        frame: u32,
        brush: isize,
        flags: u32,
    ) -> i32;
    fn GetDC(hwnd: isize) -> isize;
    fn ReleaseDC(hwnd: isize, hdc: isize) -> i32;
    fn GetSystemMetrics(index: i32) -> i32;
}

#[cfg(windows)]
#[link(name = "gdi32")]
extern "system" {
    fn GetObjectW(object: isize, size: i32, output: *mut std::ffi::c_void) -> i32;
    fn CreateCompatibleDC(hdc: isize) -> isize;
    fn CreateDIBSection(
        hdc: isize,
        info: *const BitmapInfoHeader,
        usage: u32,
        bits: *mut *mut std::ffi::c_void,
        section: isize,
        offset: u32,
    ) -> isize;
    fn GetDIBits(
        hdc: isize,
        bitmap: isize,
        start: u32,
        lines: u32,
        bits: *mut std::ffi::c_void,
        info: *mut BitmapInfoHeader,
        usage: u32,
    ) -> i32;
    fn SelectObject(hdc: isize, object: isize) -> isize;
    fn DeleteObject(object: isize) -> i32;
    fn DeleteDC(hdc: isize) -> i32;
}

#[cfg(windows)]
#[link(name = "version")]
extern "system" {
    fn GetFileVersionInfoSizeW(filename: *const u16, handle: *mut u32) -> u32;
    fn GetFileVersionInfoW(filename: *const u16, handle: u32, len: u32, data: *mut u8) -> i32;
    fn VerQueryValueW(
        block: *const u8,
        sub_block: *const u16,
        buffer: *mut *mut std::ffi::c_void,
        len: *mut u32,
    ) -> i32;
}

#[cfg(windows)]
#[link(name = "ole32")]
extern "system" {
    fn CoInitializeEx(reserved: *mut std::ffi::c_void, init: u32) -> i32;
    fn CoUninitialize();
}

#[cfg(test)]
mod tests {
    use super::{apply_and_mask, convert_bgra, encode_png, validate_program};

    #[test]
    fn zero_alpha_becomes_opaque_and_swaps_channels() {
        let rgba = convert_bgra(&[10, 20, 30, 0]);
        assert_eq!(rgba, vec![30, 20, 10, 255]);
    }

    #[test]
    fn premultiplied_white_is_expanded() {
        let rgba = convert_bgra(&[128, 128, 128, 128]);
        assert_eq!(rgba, vec![255, 255, 255, 128]);
    }

    #[test]
    fn opaque_pixel_keeps_color() {
        let rgba = convert_bgra(&[1, 2, 3, 255]);
        assert_eq!(rgba, vec![3, 2, 1, 255]);
    }

    #[test]
    fn mixed_pixels_clear_transparent_ones() {
        let rgba = convert_bgra(&[10, 20, 30, 0, 0, 0, 255, 255]);
        assert_eq!(rgba, vec![0, 0, 0, 0, 255, 0, 0, 255]);
    }

    #[test]
    fn and_mask_marks_the_high_bit_transparent() {
        let mut bgra = vec![0u8; 8 * 4];
        let mut mask = vec![0u8; 4];
        mask[0] = 0x80;
        apply_and_mask(&mut bgra, &mask, 8, 1);
        assert_eq!(bgra[3], 0);
        assert!(bgra.chunks_exact(4).skip(1).all(|pixel| pixel[3] == 255));
    }

    #[test]
    fn encode_one_pixel_png() {
        let bytes = encode_png(1, 1, &[1, 2, 3, 255]).unwrap();
        assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
    }

    #[test]
    fn reject_program_parent_component() {
        let dir = tempfile::tempdir().unwrap();
        let sneaky = dir.path().join("..").join("missing.exe");
        let err = validate_program(&sneaky.to_string_lossy()).unwrap_err();
        assert!(err.contains("不允许"));
    }

    #[test]
    fn reject_missing_program() {
        let dir = tempfile::tempdir().unwrap();
        let missing = dir.path().join("missing.exe");
        let err = validate_program(&missing.to_string_lossy()).unwrap_err();
        assert!(err.contains("不存在"));
    }

    #[test]
    fn accept_exe_file() {
        let dir = tempfile::tempdir().unwrap();
        let exe = dir.path().join("demo.exe");
        std::fs::write(&exe, b"MZ").unwrap();
        assert!(validate_program(&exe.to_string_lossy()).is_ok());
    }

    #[cfg(windows)]
    #[test]
    fn reject_non_exe_file() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("demo.txt");
        std::fs::write(&file, b"x").unwrap();
        assert!(validate_program(&file.to_string_lossy()).is_err());
    }

    #[cfg(windows)]
    #[test]
    fn notepad_icon_has_visible_detail() {
        use super::inspect_launcher;

        let path = r"C:\Windows\System32\notepad.exe";
        if !std::path::Path::new(path).is_file() {
            return;
        }
        let info = inspect_launcher(path.to_string()).expect("读取记事本信息");
        assert!(!info.name.trim().is_empty());
        assert!(info.exe_path.eq_ignore_ascii_case(path));
        let encoded = info
            .icon
            .strip_prefix("data:image/png;base64,")
            .expect("图标应为 png data url");
        let bytes = base64_decode(encoded);
        assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
        let output = std::env::temp_dir().join("ytools-notepad-icon.png");
        std::fs::write(&output, &bytes).unwrap();

        let decoder = png::Decoder::new(std::io::Cursor::new(bytes));
        let mut reader = decoder.read_info().unwrap();
        let mut buffer = vec![0u8; reader.output_buffer_size()];
        let frame = reader.next_frame(&mut buffer).unwrap();
        let pixels = &buffer[..frame.buffer_size()];
        assert!(pixels.len() >= 16);
        let opaque = pixels.chunks_exact(4).filter(|pixel| pixel[3] > 0).count();
        let distinct = {
            let mut colors = std::collections::HashSet::new();
            for pixel in pixels.chunks_exact(4) {
                if pixel[3] > 0 {
                    colors.insert([pixel[0], pixel[1], pixel[2]]);
                }
            }
            colors.len()
        };
        assert!(opaque > 8, "不透明像素太少: {opaque}");
        assert!(
            distinct > 2,
            "图标颜色太少: {distinct}，文件 {}",
            output.display()
        );
    }

    #[cfg(windows)]
    fn base64_decode(input: &str) -> Vec<u8> {
        fn value(byte: u8) -> Option<u8> {
            match byte {
                b'A'..=b'Z' => Some(byte - b'A'),
                b'a'..=b'z' => Some(byte - b'a' + 26),
                b'0'..=b'9' => Some(byte - b'0' + 52),
                b'+' => Some(62),
                b'/' => Some(63),
                _ => None,
            }
        }

        let bytes = input.as_bytes();
        let mut output = Vec::new();
        let mut index = 0;
        while index + 4 <= bytes.len() {
            let first = value(bytes[index]).unwrap_or(0);
            let second = value(bytes[index + 1]).unwrap_or(0);
            let third = value(bytes[index + 2]);
            let fourth = value(bytes[index + 3]);
            let chunk = (u32::from(first) << 18)
                | (u32::from(second) << 12)
                | (u32::from(third.unwrap_or(0)) << 6)
                | u32::from(fourth.unwrap_or(0));
            output.push((chunk >> 16) as u8);
            if third.is_some() {
                output.push((chunk >> 8) as u8);
            }
            if fourth.is_some() {
                output.push(chunk as u8);
            }
            index += 4;
        }
        output
    }
}
