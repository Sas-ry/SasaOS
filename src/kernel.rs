#![no_std]
#![no_main]
#![feature(naked_functions)]

use common::common_func::*;
use common::{print, println, read_csr, write_csr };
use common::types::*;

use core::arch::asm;
use core::ops::Index;
use core::{ptr, str};
use core::ptr::addr_of;

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

unsafe fn alloc_pages(n: u32) -> Paddr {
    if NEXT_PADDR == 0 as *mut u8 {  // 初期化チェック
        NEXT_PADDR = ptr::addr_of_mut!(__free_ram)
    }
    let paddr: Paddr = NEXT_PADDR as usize;
    NEXT_PADDR = NEXT_PADDR.add((n as usize) * PAGE_SIZE);

    if NEXT_PADDR > ptr::addr_of_mut!(__free_ram_end) {
        panic!("out of memory");  // メモリ不足でパニック
    }

    // メモリ領域をゼロ初期化
    // ptr::write_bytes(paddr as *mut u8, 0, (n as usize) * PAGE_SIZE);
    memset(paddr as *mut u8, 0, (n as usize) * PAGE_SIZE);
    paddr
}

static mut PROCCESS_CONTROLER: Process_Controler = Process_Controler::new();

#[no_mangle]
pub extern "C" fn kernel_main() {
    unsafe {
        memset(&mut __bss as *mut u8, 0, &__bss_end as *const u8 as usize - &__bss as *const u8 as usize);
        println!("\n\n");
        PROCCESS_CONTROLER.process_init();
        PROCCESS_CONTROLER.create_process(proc_a_entry as u32);
        PROCCESS_CONTROLER.create_process(proc_b_entry as u32);
        PROCCESS_CONTROLER.yield_proc();
        panic!("kernel_main");
        asm!(
            "csrw sscratch, sp",
            "addi sp, sp, -124",
            "sw ra, 0(sp)",
            "sw gp, 4(sp)",
            "sw tp, 8(sp)",
            "sw t0, 12(sp)",
            "sw t1, 16(sp)",
            "sw t2, 20(sp)",
            "sw t3, 24(sp)",
            "sw t4, 28(sp)",
            "sw t5, 32(sp)",
            "sw t6, 36(sp)",
            "sw a0, 40(sp)",
            "sw a1, 44(sp)",
            "sw a2, 48(sp)",
            "sw a3, 52(sp)",
            "sw a4, 56(sp)",
            "sw a5, 60(sp)",
            "sw a6, 64(sp)",
            "sw a7, 68(sp)",
            "sw s0, 72(sp)",
            "sw s1, 76(sp)",
            "sw s2, 80(sp)",
            "sw s3, 84(sp)",
            "sw s4, 88(sp)",
            "sw s5, 92(sp)",
            "sw s6, 96(sp)",
            "sw s7, 100(sp)",
            "sw s8, 104(sp)",
            "sw s9, 108(sp)",
            "sw s10, 112(sp)",
            "sw s11, 116(sp)",

            "csrr a0, sscratch",
            "sw a0, 120(sp)",

            "addi a0, sp, 124",
            "csrw ssratch, a0",

            "mv a0, sp",
            "call {handle_trap}",

            "lw ra, 0(sp)",
            "lw gp, 4(sp)",
            "lw tp, 8(sp)",
            "lw t0, 12(sp)",
            "lw t1, 16(sp)",
            "lw t2, 20(sp)",
            "lw t3, 24(sp)",
            "lw t4, 28(sp)",
            "lw t5, 32(sp)",
            "lw t6, 36(sp)",
            "lw a0, 40(sp)",
            "lw a1, 44(sp)",
            "lw a2, 48(sp)",
            "lw a3, 52(sp)",
            "lw a4, 56(sp)",
            "lw a5, 60(sp)",
            "lw a6, 64(sp)",
            "lw a7, 68(sp)",
            "lw s0, 72(sp)",
            "lw s1, 76(sp)",
            "lw s2, 80(sp)",
            "lw s3, 84(sp)",
            "lw s4, 88(sp)",
            "lw s5, 92(sp)",
            "lw s6, 96(sp)",
            "lw s7, 100(sp)",
            "lw s8, 104(sp)",
            "lw s9, 108(sp)",
            "lw s10, 112(sp)",
            "lw s11, 116(sp)",
            "lw sp, 120(sp)",
            "sret",
            handle_trap = sym handle_trap,
            options(noreturn)
        );
        loop {
            unsafe {
                asm!("wfi");
            }
        }
    }
}

#[no_mangle]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("PANIC: {}:{}: {} \n", file!(), line!(), _info);
    loop {}
}

unsafe extern "C" fn handle_trap(frame: *const TrapFrame) {
    let scause  = read_csr!("scause");
    let stval = read_csr!("stval");
    let user_pc = read_csr!("sepc");

    panic!("unexpected trap scause={:x}, stval={:x}, sepc={:x}", scause, stval, user_pc);
} 

extern "C" fn switch_context(prev_sp: *mut usize, next_sp: *const usize) {
    unsafe {
        asm!(
            "addi sp, sp, -13 * 4",
            "sw ra,  0  * 4(sp)",
            "sw s0,  1  * 4(sp)",
            "sw s1,  2  * 4(sp)",
            "sw s2,  3  * 4(sp)",
            "sw s3,  4  * 4(sp)",
            "sw s4,  5  * 4(sp)",
            "sw s5,  6  * 4(sp)",
            "sw s6,  7  * 4(sp)",
            "sw s7,  8  * 4(sp)",
            "sw s8,  9  * 4(sp)",
            "sw s9,  10 * 4(sp)",
            "sw s10, 11 * 4(sp)",
            "sw s11, 12 * 4(sp)",
            "sw sp, (a0)",
            "lw sp, (a1)",
            "lw ra,  0  * 4(sp)",
            "lw s0,  1  * 4(sp)",
            "lw s1,  2  * 4(sp)",
            "lw s2,  3  * 4(sp)",
            "lw s3,  4  * 4(sp)",
            "lw s4,  5  * 4(sp)",
            "lw s5,  6  * 4(sp)",
            "lw s6,  7  * 4(sp)",
            "lw s7,  8  * 4(sp)",
            "lw s8,  9  * 4(sp)",
            "lw s9,  10 * 4(sp)",
            "lw s10, 11 * 4(sp)",
            "lw s11, 12 * 4(sp)",
            "addi sp, sp, 13 * 4",
            "ret",
            options(noreturn)
        );
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Process {
    pub pid: usize,
    pub state: State,
    pub sp: Vaddr,
    pub stack: [u8; 8192],
}

impl Process {
    pub const fn new() -> Self {
        Self {
            pid: 0,
            state: State::UNUSED,
            sp: 0,
            stack: [0; 8192],
        }
    }
}

static mut CALL_A: u32 = 0;
static mut CALL_B: u32 = 0;
pub fn proc_a_entry() {
    println!("proc_a_entry\n");
    loop {
        unsafe {
            putchar('A');
            CALL_A += 1;
            PROCCESS_CONTROLER.yield_proc();
            for i in 0..30000000 {
                asm!("nop");
            }
        }
    }
}
pub fn proc_b_entry() {
    println!("proc_b_entry\n");
    loop {
        unsafe {
            putchar('B');
            CALL_B += 1;
            PROCCESS_CONTROLER.yield_proc();
            for i in 0..30000000 {
                asm!("nop");
            }
        }
    }
}

struct Process_Controler {
    procs: [Process; PROCS_MAX],
    current_idx: usize,
}

impl Process_Controler {
    pub const fn new() -> Self {
        Self {
            procs: [Process::new(); PROCS_MAX],
            current_idx: 0,
        }
    }

    pub fn process_init(&mut self) {
        unsafe {
            self.procs[0].state = State::IDLE;
            self.procs[0].pid = u32::MAX as usize;

            let stack = ptr::addr_of_mut!(self.procs[0].stack) as *mut u32;
            let sp = stack.add(self.procs[0].stack.len());
            *sp.offset(-1) = 0; // s11
            *sp.offset(-2) = 0; // s10
            *sp.offset(-3) = 0; // s9
            *sp.offset(-4) = 0; // s8
            *sp.offset(-5) = 0; // s7
            *sp.offset(-6) = 0; // s6
            *sp.offset(-7) = 0; // s5
            *sp.offset(-8) = 0; // s4
            *sp.offset(-9) = 0; // s3
            *sp.offset(-10) = 0; // s2
            *sp.offset(-11) = 0; // s1
            *sp.offset(-12) = 0; // s0
            *sp.offset(-13) = 0; // ra
            self.procs[0].sp = sp.offset(-13) as Vaddr;
        }
    }

    pub fn create_process(&mut self, pc: u32) {
        unsafe {
            let mut proc: usize = 0;
            for i in 0..PROCS_MAX {
                if self.procs[i].state == State::UNUSED {
                    proc += i; 
                    break;
                }
            }

            if (proc == 7) && (self.procs[proc].state != State::UNUSED) {
                panic!("no free process slots");
            }
            let set_proc = &mut self.procs[proc];
            let stack = ptr::addr_of_mut!(set_proc.stack) as *mut u32;
            let sp = stack.add(set_proc.stack.len());
            *sp.offset(-1) = 0; // s11
            *sp.offset(-2) = 0; // s10
            *sp.offset(-3) = 0; // s9
            *sp.offset(-4) = 0; // s8
            *sp.offset(-5) = 0; // s7
            *sp.offset(-6) = 0; // s6
            *sp.offset(-7) = 0; // s5
            *sp.offset(-8) = 0; // s4
            *sp.offset(-9) = 0; // s3
            *sp.offset(-10) = 0; // s2
            *sp.offset(-11) = 0; // s1
            *sp.offset(-12) = 0; // s0
            *sp.offset(-13) = pc; // ra
            set_proc.pid = proc as usize;
            set_proc.state = State::RUNNABLE;
            set_proc.sp = sp.offset(-13) as Vaddr;
        }
    }

    pub fn yield_proc(&mut self) {
        unsafe {
            let mut idx = 0;
            for i in 0..PROCS_MAX {
                idx = (self.current_idx + i + 1) % PROCS_MAX;
                let proc = &self.procs[idx as usize];
                if proc.state == State::RUNNABLE {
                    break;
                }
            }
    
            if self.current_idx == idx {
                return;
            }
            let stack = ptr::addr_of_mut!(self.procs[idx].stack) as *mut u32;
            let sscratch = stack.add(self.procs[idx].stack.len());
            asm!(
                "csrw sscratch, {sscratch}",
                sscratch = in(reg) sscratch,
            );
            let bef_idx = self.current_idx;
            self.current_idx = idx;
            switch_context(&mut self.procs[bef_idx].sp, &self.procs[idx].sp);
        }
    }
}