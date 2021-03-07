//! #意味着global
//! 禁用std
#![no_std]
//!
//! 不使用 'main'函数等所有Rust-level入口点作程序入口
#![no_main]


//!fn main() {
    //println!("Hello, Guo!");
//!}




use core::panic::PanicInfo;
    // ***********************************************************************
    // panic发生是调用(程序错误被迫停止或捕获机制处理)
    // Diverging Function "never"type
    // core donot need OS

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}
    // ***********************************************************************





    // ***********************************************************************
    // 覆盖crt0的_start函数
#[no_mangle]
    // 禁用编译期间 Name Managing 确保没有散化后的函数名
pub extern "C" fn _start() -> !{
    // FFI
    loop{}
}
    // ***********************************************************************







