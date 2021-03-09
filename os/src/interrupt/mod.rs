//! 中断模块 （的函数封装）-> 初始化
//!
//!

mod context;
mod handler;

pub use context::Context;

/// 初始化中断相关的子模块
///
/// - [`handler::init`]
/// - [`timer::init`]
pub fn init() {
    handler::init();
    println!("mod interrupt initialized");
}