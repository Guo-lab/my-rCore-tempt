//! 预约和处理时钟中断

use crate::sbi::set_timer;
use riscv::register::{time, sie, sstatus};


/// 时钟中断间隔，单位是CPU指令
static INTERVAL: usize = 100000;
/// 触发中断次数
pub static mut TICKS: usize = 0;



/// 初始化时钟中断
/// 开启时钟中断使能，并预约第一次时钟中断

pub fn init() {
    unsafe {
        // 开启 STIE, 允许时钟中断
        sie::set_stimer();
        // 开启 SIE (不是 sie 寄存器), 允许内核被中断打断
        // 这里的 set_sie 开启 sstatus 的 SIE 位（决定了中断是否能打断SUPERVISOR线程，为此置为1
        // 无论何值都能打断用户态线程
        sstatus::set_sie();
    }
    // 设置下一次
    set_next_timeout();
}

/* ************************************************************** */





/// 设置下一次
/// 获取当前时间，加间隔，SBI 调用预约下一次 (use::crate)
fn set_next_timeout() {
    set_timer(time::read() + INTERVAL);
}
//

/* ************************************************************** */

/// 每次中断调用
///
/// 设置下次并次数累加
pub fn tick() {
    set_next_timeout();
    unsafe { 
        // pub static mut TICKS: usize = 0;
        TICKS += 1;
        if TICKS % 100 == 0 {
            println!("{} tick", TICKS);
        }
    }
}
