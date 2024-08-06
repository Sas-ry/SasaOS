pub type Paddr = usize;
pub type Vaddr = usize;

pub const PAGE_SIZE: u32 = 4096;
pub const PROCS_MAX: usize = 8;
pub static mut PROC_UNUSED: u8 = 0;
pub static mut PROC_RUNNABLE: u8 = 1;
pub static mut NEXT_PADDR: *mut u32 = 0 as *mut u32;

extern "C" {
    pub static mut __bss: u32;
    pub static __bss_end: u32;
    pub static __stack_top: u32;
    pub static mut __free_ram: u32;
    pub static mut __free_ram_end: u32;
    pub static mut __kernel_base: u32;
}

#[repr(C)]
pub struct SbiRet {
    pub error: isize,
    pub value: isize,
}

pub struct TrapFrame {
    pub ra: u32,
    pub gp: u32,
    pub tp: u32,
    pub t0: u32,
    pub t1: u32,
    pub t2: u32,
    pub t3: u32,
    pub t4: u32,
    pub t5: u32,
    pub t6: u32,
    pub a0: u32,
    pub a1: u32,
    pub a2: u32,
    pub a3: u32,
    pub a4: u32,
    pub a5: u32,
    pub a6: u32,
    pub a7: u32,
    pub s0: u32,
    pub s1: u32,
    pub s2: u32,
    pub s3: u32,
    pub s4: u32,
    pub s5: u32,
    pub s6: u32,
    pub s7: u32,
    pub s8: u32,
    pub s9: u32,
    pub s10: u32,
    pub s11: u32,
    pub sp: u32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum State {
    UNUSED,
    RUNNABLE,
    IDLE,
    EXITED,
}



