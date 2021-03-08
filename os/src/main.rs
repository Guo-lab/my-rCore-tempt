//! #意味着global
//! 禁用std
#![no_std]
//!
//! 不使用 'main'函数等所有Rust-level入口点作程序入口
#![no_main]
//!
//! 一些 unstable 功能在 crate 层级声明之后才能用
//! 内嵌汇编
#![feature(llvm_asm)]
//!
//! 内嵌整个汇编文件
#![feature(global_asm)]
//!
//!
#![feature(panic_info_message)]
//!


#[macro_use]
mod console;
mod panic;
mod sbi;


    // ***********************************************************************
    //fn main() {
        //println!("Hello, Guo!");
    //}
    // ***********************************************************************



    // ***********************************************************************
    // 汇编语言编写的程序入口
global_asm!(include_str!("entry.asm"));
    // ***********************************************************************






/*  use core::panic::PanicInfo;
        // ***********************************************************************
        // panic发生是调用(程序错误被迫停止或捕获机制处理)
        // Diverging Function "never"type
        // core donot need OS
        //
    #[panic_handler]
    fn panic(_info: &PanicInfo) -> !{
        loop{}
    }
        // ***********************************************************************

    // ***********************************************************************
    // Make a test function to print on the screen
    // 在屏幕上输出一个字符，目前我们先不用了解其实现原理
pub fn console_putchar(ch: u8) {
    let _ret: usize;
    let arg0: usize = ch as usize;
    let arg1: usize = 0;
    let arg2: usize = 0;
    let which: usize = 1;
    unsafe {
        llvm_asm!("ecall"
             : "={x10}" (_ret)
             : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
             : "memory"
             : "volatile"
        );
    }
}
    // ***********************************************************************
*/







    // ***********************************************************************
    //    // 覆盖crt0的_start函数
    //#[no_mangle]
    //    // 禁用编译期间 Name Managing 确保没有散化后的函数名
    //pub extern "C" fn _start() -> !{
    //    // FFI
    //loop{}
    //}
    // ***********************************************************************
    //
    //
    //
#[no_mangle]
    // Rust entry function
    // _start进行了一系列准备后，这是第一个被调用的rust函数
    //
pub extern "C" fn rust_main() -> !{
//   console_putchar(b'O');
//    console_putchar(b'K');
//    console_putchar(b'K');
//    console_putchar(b'\n');
//    loop{}
//s
    println!("Hello Guo-lab!");
    panic!("rust_main END");

}





