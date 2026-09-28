use core::fmt;
use core::fmt::Write;

use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
use x86_64::instructions::interrupts::without_interrupts;

use crate::vga::color_code::{Color, ColorCode};
use crate::vga::display_char::DisplayChar;

const VGA_BUFFER: usize = 0xb8000;
const BUF_WIDTH: usize = 80;
const BUF_HEIGHT: usize = 25;

#[repr(transparent)]
pub struct Buf {
    chars: [[Volatile<DisplayChar>; BUF_WIDTH]; BUF_HEIGHT],
}

impl Buf {
    /// Returns the VGA text buffer at `0xb8000`.
    ///
    /// # Safety
    /// The buffer must be identity-mapped (the bootloader does this), and only one
    /// `&'static mut Buf` may exist at a time. Call it once, as `WRITER` does.
    #[inline]
    pub unsafe fn init() -> &'static mut Buf {
        &mut *(VGA_BUFFER as *mut Buf)
    }
}

pub struct Writer {
    pub col_pos: usize,
    pub color_code: ColorCode,
    pub prev_color: ColorCode,
    pub buf: &'static mut Buf,
}

impl Writer {
    #[inline]
    fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            // Backspace
            254 => {
                //
                // if self.col_pos <= 0 {
                //     for row in 1..BUF_HEIGHT {
                //         for col in 0..BUF_WIDTH {
                //             let character = self.buf.chars[row + 1][col].read();
                //             self.buf.chars[row][col].write(character);
                //         }
                //     }
                //     self.col_pos = 0;
                // }

                let blank = DisplayChar {
                    ascii_character: b' ',
                    color_code: self.color_code,
                };

                let (row, col) = (BUF_HEIGHT - 1, self.col_pos);
                self.buf.chars[row][col].write(blank);
                self.col_pos -= 1;
            }
            byte => {
                // Screen Wrap
                if self.col_pos >= BUF_WIDTH {
                    self.new_line();
                }

                let (row, col) = (BUF_HEIGHT - 1, self.col_pos);

                self.buf.chars[row][col].write(DisplayChar::new(byte, self.color_code));
                self.col_pos += 1;
            }
        }
    }

    #[inline]
    pub(crate) fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    #[inline]
    fn new_line(&mut self) {
        for row in 1..BUF_HEIGHT {
            for col in 0..BUF_WIDTH {
                let character = self.buf.chars[row][col].read();
                self.buf.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUF_HEIGHT - 1);
        self.col_pos = 0;
    }

    #[inline]
    fn clear_row(&mut self, row: usize) {
        let blank = DisplayChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUF_WIDTH {
            self.buf.chars[row][col].write(blank);
        }
    }

    #[inline]
    fn cls(&mut self) {
        for row in 0..BUF_HEIGHT {
            self.clear_row(row);
        }
    }

    #[inline]
    pub fn set_color(&mut self, color: ColorCode) {
        self.prev_color = self.color_code;
        self.color_code = color;
    }

    #[inline]
    pub fn restore_color(&mut self) {
        self.color_code = self.prev_color;
    }
}

impl fmt::Write for Writer {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        col_pos: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        prev_color: ColorCode::new(Color::White, Color::Black),
        buf: unsafe { Buf::init() },
    });
}

#[doc(hidden)]
#[inline]
pub fn _print(args: fmt::Arguments) {
    without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}

#[doc(hidden)]
#[inline]
pub fn _cprint(color: ColorCode, args: fmt::Arguments) {
    without_interrupts(|| {
        let mut locked = WRITER.lock();
        locked.set_color(color);
        locked.write_fmt(args).unwrap();
        locked.restore_color();
    });
}
