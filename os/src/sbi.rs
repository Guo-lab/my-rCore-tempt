// Call Machine layor
// Currently not to use all SBI call, allow unused variables and functions just for now.
#![allow(unused)]


// *************************************************************************************
// SBI 
#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}"  (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
            : "memory"         // 如果汇编可能改变memory，加入 memory Option 防止改变
            : "volatile");     // 防止汇编器激进优化
    }
    ret
}

// *************************************************************************************
//

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8; // 关机函数

// *************************************************************************************
//
//
//
// 向控制台输出一个char
// 并不能直接使用 Rust 的
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}
//
//
// 从控制态读取一个char
// 失败 return -1
pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}
//
//
//
// SBI_SHUTDOWN OS (quit QEMU)
pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    unreachable!()
}
//
//

