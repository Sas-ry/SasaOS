#![no_std]
#![no_main]

#[no_mangle]
#[link_section = ".entry"]
pub unsafe extern "C" fn _entry() {
    main();
}
#[inline]
fn main() {
    let a = 1;
    let b = 2;
    let c = a + b;

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
