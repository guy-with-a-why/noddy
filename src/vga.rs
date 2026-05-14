const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const COLS: usize = 80;
const ROWS: usize = 25;

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black      = 0x0,
    Blue       = 0x1,
    Green      = 0x2,
    Cyan       = 0x3,
    Red        = 0x4,
    Magenta    = 0x5,
    Brown      = 0x6,
    LightGray  = 0x7,
    DarkGray   = 0x8,
    LightBlue  = 0x9,
    LightGreen = 0xa,
    LightCyan  = 0xb,
    LightRed   = 0xc,
    Pink       = 0xd,
    Yellow     = 0xe,
    White      = 0xf,
}

pub fn clear() {
    for i in 0..(COLS * ROWS) {
        unsafe {
            *VGA_BUFFER.offset(i as isize * 2) = b' ';
            *VGA_BUFFER.offset(i as isize * 2 + 1) = 0x00;
        }
    }
}

pub fn print_at(text: &[u8], col: usize, row: usize, color: Color) {
    let start = (row * COLS + col) as isize;
    for (i, &byte) in text.iter().enumerate() {
        unsafe {
            *VGA_BUFFER.offset((start + i as isize) * 2) = byte;
            *VGA_BUFFER.offset((start + i as isize) * 2 + 1) = color as u8;
        }
    }
}
