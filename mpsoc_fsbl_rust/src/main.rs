#![no_std]
#![no_main]

use core::arch::global_asm;
mod csu_constants;
use csu_constants::*;

// This assembly runs first and sets up the stack pointer
global_asm!(
    ".section .text._start",
    ".global _start",
    "_start:",
    "  mov x0, #0x80000", // Example: set stack pointer below code
    "  mov sp, x0",
    "  bl main"           // Jump to our Rust main
);

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    // Your bootloader logic here!
    csu_aes_engine_reset();
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
