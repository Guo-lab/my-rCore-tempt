// 取代 std 实现 panic abort 
//
use core::panic::PanicInfo;
use crate::sbi::shutdown;
//
// ***********************************************************************
// 打印 panic 并SHUTDOWN
// 声明此函数是 panic 的回调
//
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    //
    // '\x1b[___m' 控制终端字符输出格式的命令（此为红色）
    // 需要全局开启 feature(panic_info_message) to call .message()
    //
    println!("\x1b[1;31mpanic: '{}'\x1b[0m", info.message().unwrap());
    shutdown();
}


// Finish programe running
//
// Call panic_handler
//
#[no_mangle]
extern "C" fn abort() -> ! {
    panic!("abort()!")
}
