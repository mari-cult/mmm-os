// Disable Rust's standard library and main entry point
// Required for bare-metal development
#![no_main]
#![no_std]

mod exceptions;
mod gic;
mod sync;
mod uart;
mod utilities;

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
pub extern "C" fn kmain(_fdt_addr: usize) {
	uart::print(b"MMM!\n");
	loop {
		if let Some(ch) = uart::getchar() {
			uart::print(b"You typed: ");
			uart::putchar(ch);
			uart::print(b"\n");
		}
	}
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
	uart::print(b"Panic!\n");
	loop {}
}
