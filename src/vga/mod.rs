use volatile::Volatile;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

//marco magic that's from std
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    x86_64::instructions::interrupts::without_interrupts(|| { // this prevents interrupts from causing a deadlock
        WRITER.lock().write_fmt(args).unwrap();//write_fmt is from the write impl that we did
    });
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum Color {
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
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    char: u8,
    color: ColorCode,
}

impl ScreenChar {
    fn new(character: u8, color_code: ColorCode) -> ScreenChar {
        ScreenChar{
            char: character,
            color: color_code,
        }
    }
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer{
    col_pos: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new(color: ColorCode) -> Writer {
        Writer{
            col_pos: 0,
            color_code: color,
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col_pos >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;//this is the bottom of the screen because the new_line function shuffles everything up by one
                let col = self.col_pos;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar::new(byte, color_code));
                self.col_pos += 1;
            }
        }
    }
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let char = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(char);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.col_pos = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar::new(b' ', self.color_code);
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte), //writable range and newline is just written
                _ => self.write_byte(0xfe), //everything else is written as a block
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

//lazy static allows the initilization of static things at run time instead of compile time and the
//mutex is a non-std way of making sure that 2 things in the program aren't writting to the vga
//buffer at the same time so it's not all gobbilty gook
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(ColorCode::new(Color::Yellow, Color::Black)));
}

//TESTS

#[allow(unused_imports)]
#[cfg(test)]
use crate::{
    serial_print,
    serial_println,
};

//check to see if the println macro panics at all
#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

//make sure the scrolling doesn't cause anything to panic
#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_simple output");
    }
}

//make sure the println macro is actually writing to the vga interface and not just.... doing
//nothing I guess
#[test_case]
fn test_println_write() {
    let s = "A test string that will be on a line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.char), c);
    }
}

