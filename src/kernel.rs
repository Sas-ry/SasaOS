#![no_std]
#![no_main]
#![feature(naked_functions)]

use common::common_func::*;
use common::{print, println, read_csr, write_csr };
use common::types::*;

use core::arch::asm;

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
        // common_func::memset(&mut __bss as *mut u8, 0, &__bss_end as *const u8 as usize - &__bss as *const u8 as usize);
        println!("Hello,{}!", "World");
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

