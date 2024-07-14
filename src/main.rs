use std::arch::asm;

fn main() {
    let mut result: u64;
    let a: u64 = 5;
    let b: u64 = 3;

    unsafe {
        asm!(
            "add {0}, {1}, {2}",
            out(reg) result,
            in(reg) a,
            in(reg) b
        );
    }

    println!("Result of {} + {} is: {}", a, b, result);
}
