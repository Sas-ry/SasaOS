#![no_std]
#![no_main]
#![feature(asm)]
#![feature(naked_functions)]
#![feature(global_asm)]

use core::arch::asm;
use core::arch::global_asm;

type uint8_t = u8;
type uint32_t = u32;
type size_t = uint32_t;

extern "C" {
    static mut __bss: uint8_t;
    static mut __bss_end: uint8_t;
    static mut __stack_top: uint8_t;
}

#[no_mangle]
unsafe fn memset(buf: *mut uint8_t, c: uint8_t, n: size_t) -> *mut u8 {
    let mut p = buf;
    for _ in 0..n {
        *p = c;
        p = p.add(1);
    }
    buf
}
#[naked]
#[no_mangle]
pub extern "C" fn boot() -> ! {
    unsafe {
        asm!(
            "la sp, {stack_top}",
            "j kernel_main",
            stack_top = sym __stack_top,
            options(noreturn)
        );
    }
}

#[no_mangle]
pub extern "C" fn kernel_main() {
    unsafe {
        memset(&mut __bss as *mut u8, 0, &__bss_end as *const u8 as size_t - &__bss as *const u8 as size_t);
    }

    loop {}
}

#[no_mangle]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
