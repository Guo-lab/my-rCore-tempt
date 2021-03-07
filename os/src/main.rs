//! #意味着global
//! 禁用std

#![no_std]



fn main() {
    //println!("Hello, Guo!");
}




use core::panic::PanicInfo;
    // *************************************************************************************
    // panic发生是调用(程序错误被迫停止或捕获机制处理)
    // Diverging Function "never"type
    // core donot need OS

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}
    // *************************************************************************************
