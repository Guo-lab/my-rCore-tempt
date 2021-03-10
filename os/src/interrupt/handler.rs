///为了让硬件能够找到__interrupt Entry，OS 初始化写入 stvec Reg
use super::context::Context;
use riscv::register::stvec;
use riscv::register::scause::Scause;
use riscv::register::scause::Trap;
use riscv::register::scause::Exception;
use riscv::register::scause::Interrupt;

//
// WRONG[use riscv::register::scause::timer;]
// no `timer` in `register::scause`
use super::timer;


global_asm!(include_str!("./interrupt.asm"));

//
//
//
//
//
/// 初始化中断处理
///
/// 把中断入口 `__interrupt` 写入 `stvec` 中，并且开启中断使能
pub fn init() {
    unsafe {
        extern "C" {
            /// `interrupt.asm` 中的中断入口
            fn __interrupt();
        }
        // 使用 Direct 模式，将中断入口设置为 `__interrupt`
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);
    }
}

//
//
//
//
//
//

/// 2021-3-10 写入timer.rs后补充根据不同中断种类进行不同处理 
/// 中断的处理入口
///
/// `interrupt.asm` 首先保存寄存器至 Context，其作为参数和 scause 以及 stval 一并传入此函数
/// 具体的中断类型需要根据 scause 来推断，然后分别处理
//
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    //panic!("Interrupted: {:?}",scause.cause());
    // ****************************************
    // Debug 查看中断种类
    // println!("{:x?}", scause.cause());
    // ****************************************
    match scause.cause(){
        // 断点中断（ebreak）
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        // 时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        // *其他情况，无法处理*
        _ => fault(context, scause, stval),
    }
}
//
//
//
//
/// 处理 EBREAK
///
/// 继续执行， 其中 'sepc' 增加两个字节，以跳过该条EBREAK指令
fn breakpoint(context: &mut Context) {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;
}
//
//
//
//
//
/// 处理时钟中断 目前只在 ['timer'] 模块中计数
fn supervisor_timer(_: &mut Context) {
    timer::tick();
}
//
//
///出现未能解决的异常
fn fault(context: &mut Context, scause: Scause, stval: usize) {
    panic!(
        "Unresolved interrupt: {:?}\n{:x?}\nstval: {:x}",
        scause.cause(),
        context,
        stval
    );    
}




