pub type Uint8T = u8;
pub type Uint32T = u32;
pub type SizeT = Uint32T;

extern "C" {
    pub static mut __bss: Uint8T;
    pub static mut __bss_end: Uint8T;
    pub static mut __stack_top: Uint8T;
}