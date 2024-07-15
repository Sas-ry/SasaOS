use core::arch::asm;

#[repr(C)]
pub struct SbiRet {
    error: isize,
    value: isize,
}

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