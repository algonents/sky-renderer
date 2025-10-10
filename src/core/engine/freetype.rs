use std::ffi::{c_char, c_int, c_long, c_uchar, c_uint, c_ulong, c_ushort, c_void};
use std::slice;

#[repr(C)]
pub struct FT_LibraryRec_;
pub type FT_Library = *mut FT_LibraryRec_;

#[repr(C)]
pub struct FT_FaceRec_;
pub type FT_Face = *mut FT_FaceRec_;

pub type FT_Error = c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GlyphDimensions {
    pub width: c_int,
    pub height: c_int,
    pub left: c_int,
    pub top: c_int,
    pub advance_x: c_int, // 26.6 fixed
}

#[repr(C)]
#[derive(Debug)]
pub struct FT_Bitmap {
    pub rows: c_uint,
    pub width: c_uint,
    pub pitch: c_int,
    pub buffer: *mut c_uchar,
    pub num_grays: c_ushort,
    pub pixel_mode: c_uchar,
    pub palette_mode: c_uchar,
    pub palette: *mut c_void,
}

pub const FT_PIXEL_MODE_MONO: u8 = 1;
pub const FT_PIXEL_MODE_GRAY: u8 = 2;
pub const FT_PIXEL_MODE_LCD:  u8 = 5;
pub const FT_PIXEL_MODE_LCD_V: u8 = 6;

unsafe extern "C" {
    pub fn _ftInitFreeType() -> FT_Error;
    pub fn _ftDoneFreeType() -> FT_Error;

    pub fn _ftNewFace(filePath: *const c_char, faceIndex: c_long, aface_: *mut FT_Face) -> FT_Error;
    pub fn _ftDoneFace(aface: FT_Face) -> FT_Error;

    pub fn _ftSetPixelSizes(face: FT_Face, pixel_width: c_uint, pixel_height: c_uint) -> FT_Error;
    pub fn _ftLoadChar(face: FT_Face, char_code: c_ulong, load_flags: c_int) -> FT_Error;

    pub fn _getGlyphDimensions(face: FT_Face, dimensions: *mut GlyphDimensions) -> FT_Error;
    pub fn _getBitmap(face: FT_Face, out_bitmap: *mut *mut FT_Bitmap) -> FT_Error;
}

#[inline]
pub fn advance26_6_to_px(x26_6: c_int) -> f32 { (x26_6 as f32) / 64.0 }

/// SAFETY: Valid only until the next `_ftLoadChar` on the same face.
pub unsafe fn bitmap_as_bytes(bmp: &FT_Bitmap) -> Option<&[u8]> {
    if bmp.buffer.is_null() { return None; }
    let len = (bmp.rows as isize * bmp.pitch.abs() as isize) as usize;
    Some(slice::from_raw_parts(bmp.buffer as *const u8, len))
}
