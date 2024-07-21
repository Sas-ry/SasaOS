use core::str;
use crate::common_func::putchar;

type Paddr = u32;
type Vaddr = u32;

extern "C" {
    pub static mut __bss: u8;
    pub static mut __bss_end: u8;
    pub static mut __stack_top: u8;
}

#[repr(C)]
pub struct SbiRet {
    pub error: isize,
    pub value: isize,
}

pub struct Writer;

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            putchar(c)
        }
        Ok(())
    }
}