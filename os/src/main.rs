//! # 一些 unstable 的功能需要在 crate 层级声明后才可以使用
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
//! 2021-3-10
//!   我们使用了一个全局动态内存分配器，以实现原本标准库中的堆内存分配。
//!   而语言要求我们同时实现一个错误回调[panic!()]
#![feature(alloc_error_handler)]
//!








//  // ***********************************************************************
#[macro_use]
mod console;
mod panic;
mod sbi;
// 中断模块
mod interrupt;

// 内存模块的引入 还需 memory/mod.rs 支持
mod memory;
extern crate alloc;
//






    /* ***********************************************************************
src[]fn main() {
        println!("Hello, Guo!");
     }
    *********************************************************************** */




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
}// ***********************************************************************
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




    // ***********************************************************************
    // Rust entry function
    // _start进行了一系列准备后，这是第一个被调用的rust函数
    //
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    /* console_putchar(b'O');
       console_putchar(b'K');
       console_putchar(b'K');
       console_putchar(b'\n');
         loop{}
    */
   


    println!("Hello Guo-lab!");
    //  初始化各种模块
    interrupt::init();
    memory::init();




// 测试动态内存分配
/*  use alloc::boxed::Box;
    use alloc::vec::Vec;
    let v = Box::new(5);
    assert_eq!(*v, 5);
    core::mem::drop(v);

    let mut vec = Vec::new();
    for i in 0..10000 { 
        vec.push(i);
    }
    assert_eq!(vec.len(), 10000);
    for (i,value) in vec.into_iter().enumerate() {
        assert_eq!(value,i);
    }
    println!("Heap Test Passed!");
    panic!(); */
// 测试动态内存分配结束


// 测试中断
//    unsafe {
//        llvm_asm!("ebreak"::::"volatile");
//    };
    // *******************************************************************
    // 2021-3-10 __tick__ (一处修改：上下文处context 添加mut FILE interrupt）
    // // unreachable!();
    // #################
    // loop{}
    // #################
    // 2021-3-10
    // 时钟中断需要在loop中进行
    //   [ Rust使用的是代数类型系统，!表示最小的类型单元，类似离散数学里的零元
    // 因此没有unreachable()!与之对应不能使用
    // ********************************************************************
    // 
// 测试结束
   


/* 测试 KNERNEL_ADDRESS 2021-3-11
// 注意这里的KERNEL_END_ADDRESS 为 ref 类型，需要加 *
    println!("{}", *memory::config::KERNEL_END_ADDRESS);
    panic!()
// 测试结束 */



// TEST  frame_Tracker ALLOCATOR  2021-3-12
    // PhysicalPage Allocate
    for _ in 0..2 {
        let frame0_ = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}",err)
        };
        let frame1_ = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}",err)
        };
        println!("{} and {}", frame0_.address(), frame1_.address());
    }
    panic!("Successfully finish frame")
// TEST DONE




//  panic!("rust_main END");
// All is over here
}





