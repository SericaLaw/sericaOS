use crate::riscv::sbi;
use crate::device::uart;
use core::fmt::{self, Write};

struct StdOut;

impl fmt::Write for StdOut {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        puts(s);
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    StdOut.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::io::_print(format_args!($($arg)*));
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn putchar(ch: char) {
    sbi::console_putchar(ch as u8 as usize);
}

pub fn puts(s: &str) {
    for ch in s.chars() {
        putchar(ch);
    }
}


#[macro_export]
macro_rules! uart_print {
	($($args:tt)+) => ({
			use core::fmt::Write;
			let _ = write!($crate::device::uart::Uart::new(0x1000_0000), $($args)+);
	});
}
#[macro_export]
macro_rules! uart_println {
	() => ({
		uart_print!("\r\n")
	});
	($fmt:expr) => ({
		uart_print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		uart_print!(concat!($fmt, "\r\n"), $($args)+)
	});
}