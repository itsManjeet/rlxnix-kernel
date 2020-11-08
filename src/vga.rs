#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
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


const VGA_HEIGHT: usize = 25;
const VGA_WIDTH: usize = 80;


use volatile::Volatile;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<u16>; VGA_WIDTH]; VGA_HEIGHT],
}

pub struct Vga {
    buffer: &'static mut Buffer,
    index: usize,
    color: u8,
}

impl Vga {

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),

            byte => {
                let x = self.index % VGA_WIDTH;
                let y = self.index / VGA_WIDTH;
                let color = self.color;

                self.buffer.chars[y][x].write((byte as u16) | (color as u16) << 8);

                self.index += 1;
            }
        }
    }

    fn new_line(&mut self) {
        self.index += VGA_WIDTH - (self.index % VGA_WIDTH);
        if self.index >= VGA_WIDTH * VGA_HEIGHT {
            self.index = 0;
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xFE),
            }
        }
    }
}


use core::fmt;

impl fmt::Write for Vga {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref VGA: Mutex<Vga> =  Mutex::new(Vga {
        index: 0,
        color: (Color::Cyan as u8) | (Color::Black as u8) << 4,
        buffer: unsafe { &mut *(0xB8000 as *mut Buffer) }
    });
}



#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n")};
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    VGA.lock().write_fmt(args).unwrap();
}