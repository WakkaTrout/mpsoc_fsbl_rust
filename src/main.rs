#![no_std]
#![no_main]

use core::arch::global_asm;
mod csu_constants;
use csu_constants::*;
mod crl_apb_constants;
use crl_apb_constants::*;

// This assembly runs first and sets up the stack pointer
global_asm!(
    ".section .text._start",
    ".global _start",
    "_start:",
    "  mov x0, #0x80000", // TODO: Change this to use stack pointer from linker script
    "  mov sp, x0",
    "  bl main"           // Jump to our Rust main
);

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    // Your bootloader logic here!
    csu_aes_engine_reset();
    csu_sha_engine_reset();

    let system_reset_reason : u32 = crl_apb_get_reset_reason();
    let boot_mode : BootMode = crl_apb_get_user_boot_mode();
    if boot_mode != BootMode::JtagBootMode
    {
        crl_apb_set_user_alt_boot_mode(BootMode::JtagBootMode);
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
