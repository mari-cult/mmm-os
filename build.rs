use cc::Build;
use std::env;

fn main() {
	unsafe {
		env::set_var("CROSS_COMPILE", "aarch64-unknown-none-elf");
		env::set_var("CC", "clang");
	}

	Build::new()
		.file("src/boot.s")
		.file("src/interrupt.s")
		.asm_flag("-Isrc")
		.flag("-Wno-unused-command-line-argument")
		.compile("empty")
}
