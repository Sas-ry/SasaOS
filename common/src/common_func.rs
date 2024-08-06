use core::arch::asm;
use crate::types::*;

use core::fmt::Write;
use core::fmt::{self, Error};
use core::ptr;

/* 
SBIの仕様に沿ってOpenSBIを呼び出すための関数
呼び出し規約は以下を参考
ref：https://github.com/riscv-non-isa/riscv-sbi-doc
すべての SBI 関数は、単一のバイナリ・エンコーディングを共有するため、SBI 拡張の混在が容易になります。SBI 仕様では、以下の呼び出し規約に従っています。
- ECALLは、スーパーバイザとSEE間の制御転送命令として使用される。
- a7 は、SBI 拡張 ID（EID）を符号化する
- a6 は、SBI v0.2 以降に定義された SBI 拡張について、a7 でエンコードされた所定の拡張 ID の SBI 機能 ID（FID）をエンコードする。
- a0とa1を除くすべてのレジスタは、着呼側によってSBI呼び出しの間保持されなければならない。
    →a2からa7までのレジスタの値は呼び出し後もそのままであることが保証される
- SBI関数は、a0とa1の値のペアを返す必要があり、a0はエラーコードを返す。これは、C言語の構造体を使用して返される。
*/
#[no_mangle]
pub fn sbi_call(arg0: isize, arg1: isize, arg2: isize, arg3: isize, arg4: isize, arg5: isize, fid: isize, eid: isize) -> SbiRet {
    let error: isize;
    let value: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("a0") arg0 => error,
            inlateout("a1") arg1 => value,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") eid,
            options(nostack)
        );
    }
    SbiRet { error, value }
}

/* 
Open SBIのConsole Putchar関数を呼び出すための関数
chに存在するデータをデバッグコンソールに書き込む
sbi_console_getchar()とは異なり、このSBIコールは、送信すべき保留中の文字が残っている場合、
または受信端末がまだバイトを受信する準備ができていない場合、ブロックされる。
しかし、コンソールが全く存在しない場合、その文字は捨てられる。
*/
#[no_mangle]
pub fn putchar(ch: char) {
    sbi_call(ch as isize, 0, 0, 0, 0, 0, 0, 1);
}

struct SimpleWriter;

#[no_mangle]
pub fn memset(buf: *mut u32, c: u32, n: usize) -> *mut u32 {
    let mut p = buf;
    unsafe {
        for _ in 0..n {
            *p = c;
            p = p.add(1);
        }
    }
    buf
}

#[no_mangle]
pub fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    unsafe {
        let mut d = dst;
        let mut s = src;
        let mut count = n;
        while count != 0 {
            *d = *s;
            d = d.add(1);
            s = s.add(1);
            count -= 1;
        }
        dst
    }
}

/* 
    strcpy関数はdstのメモリ領域よりsrcの方が長い時でも、dstのメモリ領域を終えてコピーを行うため、バグや脆弱性に繋がりやすい
    余力があれば代替関数のstrcpy_sを実装したい
*/
#[no_mangle]
pub fn strcpy(dst: *mut u8, src: *const u8) -> *mut u8 {
    let mut d = dst;
    let mut s = src;
    
    unsafe {
        // ソースがヌル文字に到達するまで繰り返す
        while *s != 0 {
            // 安全でないポインタ操作を行う
            ptr::write(d, ptr::read(s));
            d = d.add(1);
            s = s.add(1);
        }

        // 最後にヌル文字を追加
        ptr::write(d, b'0');
    }
    dst
}

#[no_mangle]
pub unsafe fn strcmp(s1: *const u8, s2: *const u8) -> i32 {
    let mut a = s1;
    let mut b = s2;
    
    // 両方のポインタが指す値を比較し、どちらかがヌル文字に達するまで繰り返す
    while *a != 0 && *b != 0 {
        if *a != *b {
            break;
        }
        a = a.add(1);
        b = b.add(1);
    }

    (*a as i32) - (*b as i32)
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

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!(crate::Writer, $($arg)*);
    });
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ({
        print!("{}\n", format_args!($($arg)*));
    });
}

// CSR を読み書きするマクロ
#[macro_export]
macro_rules! read_csr {
    ($csr:expr) => {
        unsafe {
            let mut csrr: u32;
            asm!(concat!("csrr {r}, ", $csr), r = out(reg) csrr);
            csrr
        }
    };
}

#[macro_export]
macro_rules! write_csr {
    ($reg:expr, $value:expr) => {{
        unsafe { asm!("csrw {}, {}", const $reg, in(reg) $value) };
    }};
}

#[macro_export]
macro_rules! is_aligned {
    ($addr:expr, $align:expr) => {
        $align.is_power_of_two() && ( $addr % $align == 0 )
    };
}

unsafe extern "C" fn handle_trap(frame: *const TrapFrame) {
    let scause  = read_csr!("scause");
    let stval = read_csr!("stval");
    let user_pc = read_csr!("sepc");

    panic!("unexpected trap scause={:x}, stval={:x}, sepc={:x}", scause, stval, user_pc);
}
