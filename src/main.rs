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

#[repr(C)]
struct SbiRet {
    error: usize,
    value: usize,
}

#[no_mangle]
fn sbi_call(arg0: usize, arg1: usize, arg2: usize, arg3: usize, arg4: usize, arg5: usize, fid: usize, eid: usize) -> SbiRet {
    let error: usize;
    let value: usize;
    unsafe {
        asm!(
            "ecall",
            inlateout("a0") arg0 => error,
            inlateout("a1") arg1 => value,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") eid,
            options(nostack)
        );
    }
    SbiRet { error, value }
}

#[no_mangle]
fn putchar(ch: uint8_t) {
    sbi_call(ch as usize, 0, 0, 0, 0, 0, 0, 1);
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
