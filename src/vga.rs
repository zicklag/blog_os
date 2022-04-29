//! VGA buffer access

const VGA_BUFFER_MEM_ADDR: *mut u8 = 0xb8000 as *mut u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[allow(dead_code)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> Self {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    pub asci_char: u8,
    pub color: ColorCode,
}

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    pub colum_position: usize,
    pub color: ColorCode,
    pub buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new() -> Self {
        Writer {
            colum_position: 0,
            color: ColorCode::new(Color::LightGray, Color::Black),
            buffer: unsafe { &mut *(VGA_BUFFER_MEM_ADDR as *mut Buffer) },
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_char(byte as char),
                // not part of printable ASCII range
                _ => self.write_char(0xfe as char),
            }
        }
    }

    pub fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.new_line(),
            c => {
                if self.colum_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = BUFFER_HEIGHT - 1;
                let col = self.colum_position;
                let color = self.color;
                self.buffer.chars[row][col] = ScreenChar {
                    asci_char: c as u8,
                    color,
                };
                self.colum_position += 1;
            }
        }
    }

    fn new_line(&mut self) {}
}
