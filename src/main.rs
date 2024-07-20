#![no_std]
#![no_main]
#![feature(naked_functions)]

#[macro_use]
mod common_func;
mod types;
use core::arch::asm;
use core::arch::global_asm;
use common_func::printf_test;
use types::*;

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
        common_func::memset(&mut __bss as *mut u8, 0, &__bss_end as *const u8 as usize - &__bss as *const u8 as usize);
        printf_test("\n\nHello %s\n%d\n", &[&"World!", &"11"]);
        panic_test();
    }
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
fn panic_test() {
    let mut buf = [0u8; 11]; // u32の最大値は10桁
    let mut temp = line!();
    let mut i = 0;

    // 数値を文字に変換してバッファに格納
    while temp > 0 {
        let digit = (temp % 10) as u8 + b'0';
        buf[i] = digit;
        temp /= 10;
        i += 1;
    }

    // バッファを逆順にする
    buf[..i].reverse();

    // 有効な部分を文字列スライスとして取得
    let s = core::str::from_utf8(&buf[..i]).unwrap();
    printf_test("PANIC: %s:%d: \n", &[&file!(), s]);
    loop {}
}

#[no_mangle]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    let mut buf = [0u8; 11]; // u32の最大値は10桁
    let mut temp = line!();
    let mut i = 0;

    // 数値を文字に変換してバッファに格納
    while temp > 0 {
        let digit = (temp % 10) as u8 + b'0';
        buf[i] = digit;
        temp /= 10;
        i += 1;
    }

    // バッファを逆順にする
    buf[..i].reverse();

    // 有効な部分を文字列スライスとして取得
    let s = core::str::from_utf8(&buf[..i]).unwrap();
    printf_test("PANIC: %s:%d: \n", &[&file!(), s]);
    loop {}
}
