#![no_std]
#![no_main]

use core::arch::asm;

#[no_mangle]
#[link_section = ".boot"]
pub unsafe extern "C" fn _boot() {
    main();
}
#[inline]
fn main() {
    let result: i32;
    unsafe {
        asm!(
            "mov {0}, 1",
            "add {0}, 2",
            out(reg) result
        );
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
