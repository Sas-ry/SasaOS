#![no_std]
#![no_main]
#![feature(naked_functions)]

use common::common_func::*;
use common::{print, println, read_csr, write_csr };
use common::types::*;

use core::arch::asm;
use core::{ptr, str};

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

#[no_mangle]
pub extern "C" fn kernel_main() {
    unsafe {
        memset(&mut __bss as *mut u8, 0, &__bss_end as *const u8 as usize - &__bss as *const u8 as usize);
        let paddr0 = alloc_pages(1);
        println!("paddr0: {:x}", paddr0);
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

unsafe fn switch_context(prev_sp: u32, next_sp: u32) {
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
        "sw sp, {prev_sp}",
        "lw sp, {next_sp}",
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
        prev_sp = in(reg) prev_sp,
        next_sp = in(reg) next_sp,
        options(noreturn)
    );
}

struct ProcessManager {
    procs: [Process; PROCS_MAX],
    pub current: usize,
}

//impl ProcessManager {
    //fn create_process(pc: u32) -> Process {
        //unsafe {
            //let mut process_slot = None;
            //for i in 0..PROCS_MAX {
                //if Self::procs[i].state == PROC_UNUSED {
                    //process_slot = Some(i);
                    //break;
                //}
            //}
    
            //let i = process_slot.expect("no free process slots");
    
            //let process = &mut PROCS[i];
            //let mut sp = process.stack.len();
    
            //// push registers s11 to s0 and return address
            //for _ in 0..12 {
                //sp -= 1;
                //process.stack[sp] = 0;
            //}
            //sp -= 1;
            //process.stack[sp] = pc;
    
            //// Setup process fields
            //process.pid = i + 1;
            //process.state = ProcessState::Runnable;
            //process.sp = sp;
    
            //process
        //}
    //}
//}