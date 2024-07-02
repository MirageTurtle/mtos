// Code for the VGA text buffer
use core::fmt;
use lazy_static::lazy_static;
use volatile::Volatile;

// Implement the fmt::Write trait for supporting Rust's formatting macros
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// colors using an enum
#[allow(dead_code)] // disable the warning for unused code
#[derive(Debug, Clone, Copy, PartialEq, Eq)] // printable, copyable, comparable
#[repr(u8)] // rust doesn't have `u4` so we use `u8` instead
pub enum Color {
    Black = 0,       // 0x0
    Blue = 1,        // 0x1
    Green = 2,       // 0x2
    Cyan = 3,        // 0x3
    Red = 4,         // 0x4
    Magenta = 5,     // 0x5
    Brown = 6,       // 0x6
    LightGray = 7,   // 0x7
    DarkGray = 8,    // 0x8
    LightBlue = 9,   // 0x9
    LightGreen = 10, // 0xA
    LightCyan = 11,  // 0xB
    LightRed = 12,   // 0xC
    Pink = 13,       // 0xD
    Yellow = 14,     // 0xE
    White = 15,      // 0xF
}

// a full color byte
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // has the same memory layout as u8
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

// a screen character
/**
 * VGA text buffer
 * 0-7: ASCII code point
 * 8-11: foreground color
 * 12-14: background color
 * 15: blink
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // the struct should have a C-like layout
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// A writer for actually writing to the VGA buffer
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer, // the `'static` lifetime
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1; // just write in the bottom line
                let col = self.column_position;

                let color_code = self.color_code;
                // Abandoned the following code because of Volatile type
                // self.buffer.chars[row][col] = ScreenChar {
                //     ascii_character: byte,
                //     color_code,
                // };
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn new_line(&mut self) {
        // move all characters one row up
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[row - 1][col].write(self.buffer.chars[row][col].read());
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    pub fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte), // printable ASCII byte or newline
                _ => self.write_byte(0xfe),                   // unknown character
            }
        }
    }
}

pub fn print_something() {
    use core::fmt::Write;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_string("Hello, Wörld!\n"); // Hello, W■■rld!
    write!(writer, "The numbers are {} and {}", 42, 1.0 / 3.0).unwrap();
}

// the global writer instance
lazy_static! {
    pub static ref WRITER: Writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
}
