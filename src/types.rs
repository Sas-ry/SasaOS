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
