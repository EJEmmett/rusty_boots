use core::fmt::Write;
use lazy_static::lazy_static;
use spin::Mutex;
use uart_16550::SerialPort;

pub struct SerialAccess {
    qemu: SerialPort,
    bochs: SerialPort,
}

impl Default for SerialAccess {
    fn default() -> Self {
        Self::new()
    }
}

impl SerialAccess {
    #[inline]
    pub fn new() -> Self {
        let mut qemu = unsafe { SerialPort::new(0x3F8) };
        qemu.init();

        let mut bochs = unsafe { SerialPort::new(0xe9) };
        bochs.init();

        Self { qemu, bochs }
    }

    #[inline]
    pub fn write(&mut self, args: ::core::fmt::Arguments) {
        self.qemu.write_fmt(args).expect("Writing to qemu failed.");
        self.bochs
            .write_fmt(args)
            .expect("Writing to bochs failed.");
    }
}

lazy_static! {
    pub static ref SERIAL: Mutex<SerialAccess> = Mutex::new(SerialAccess::new());
}

#[doc(hidden)]
#[inline]
pub fn _print(args: ::core::fmt::Arguments) {
    use x86_64::instructions::interrupts::without_interrupts;
    without_interrupts(|| {
        SERIAL.lock().write(args);
    });
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($ ($arg) *));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ( $($arg:tt)* ) => ($crate::serial_print!("({}::{}) {}\n", module_path!(), line!(), format_args!($ ($arg) *)));
}
