//! print to serial port
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::port::PortWriteOnly;

/// Minimal serial port
pub struct SerialPort {
    data: PortWriteOnly<u8>,
}

impl SerialPort {
    /// Creates a new serial port interface on the given I/O port.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the caller must ensure that the given base address
    /// really points to a serial port device.
    pub const unsafe fn new(base: u16) -> SerialPort {
        SerialPort {
            data: PortWriteOnly::<u8>::new(base),
        }
    }

    /// Sends a byte on the serial port.
    #[inline]
    pub fn send(&mut self, data: u8) {
        unsafe {
            match data {
                8 | 0x7F => {
                    self.data.write(8);
                    self.data.write(b' ');
                    self.data.write(8)
                }
                _ => {
                    self.data.write(data);
                }
            }
        }
    }
}

impl fmt::Write for SerialPort {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.send(byte);
        }
        Ok(())
    }
}

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let serial_port = unsafe { SerialPort::new(0x3F8) };
        //serial_port.init();
        Mutex::new(serial_port)
    };
}

lazy_static! {
    pub static ref SERIAL2: Mutex<SerialPort> = {
        let serial_port = unsafe { SerialPort::new(0x2f8) };
        //serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}

#[doc(hidden)]
pub fn _eprint(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        SERIAL2
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::arch::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_eprint {
    ($($arg:tt)*) => {
        $crate::arch::serial::_eprint(format_args!($($arg)*));
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}

#[macro_export]
macro_rules! print {
    () => ();
    ($fmt:expr) => ($crate::serial_print!($fmt));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!($fmt, $($arg)*));
}

#[macro_export]
macro_rules! eprintln {
    () => ($crate::serial_eprint!("\n"));
    ($fmt:expr) => ($crate::serial_eprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_eprint!(concat!($fmt, "\n"), $($arg)*));
}
