#![no_std]
#![no_main]
#![feature(naked_functions)]

mod sbi_call;
mod types;
use core::arch::asm;
use core::arch::global_asm;
use types::*;

#[no_mangle]
fn putchar(ch: Uint8T) {
    sbi_call::sbi_call(ch as isize, 0, 0, 0, 0, 0, 0, 1);
}

#[no_mangle]
unsafe fn memset(buf: *mut Uint8T, c: Uint8T, n: SizeT) -> *mut u8 {
    let mut p = buf;
    for _ in 0..n {
        *p = c;
        p = p.add(1);
    }
    buf
}
#[naked]
#[no_mangle]
#[link_section = ".text.boot"]
pub extern "C" fn boot() -> ! {
    unsafe {
        asm!(
            "la sp, {stack_top}\n",
            "j kernel_main\n",
            stack_top = sym __stack_top,
            options(noreturn)
        );
    }
}

#[no_mangle]
pub extern "C" fn kernel_main() {
    unsafe {
        //memset(&mut __bss as *mut u8, 0, &__bss_end as *const u8 as size_t - &__bss as *const u8 as size_t);
        let s = b"\n\nHello World!\n";
        for &ch in s {
            putchar(ch);
        }
    }
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

#[no_mangle]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
