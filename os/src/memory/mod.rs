//! 内存管理模块
//!
//! 负责空间分配和虚拟地址映射

// 因为模块内包含许多基础设施类别，实现了许多以后可能会用到的函数，
// 所以在模块范围内不提示「未使用的函数」等警告
#![allow(dead_code)]





// 2021-3-12
/// 一个缩写，模块中一些函数会使用
pub type MemoryResult<T> = Result<T, &'static str>;
/* Example
  src/memory/frame/allocator.rs:58:45
     |
  58 |     pub fn alloc(&mut self) -> MemoryResult<FrameTracker> {
*/




// 2021-3-10
pub mod config;
pub mod heap;
// 2021-3-11
pub mod address;
// 2021-3-12
// mod frame; //  ^^^^^ private module
pub mod frame;
pub mod range;

// 2021-3-15
// to use pub mod mapping if not mapping.rs
// I should have mod.rs in mapping
pub mod mapping;




pub use {
// 2021-3-10
    config::*,
// 2021-3-12
    address::*,
// 2021-3-12
    frame::FRAME_ALLOCATOR,
    range::Range,
};





/// 初始化内存相关的子模块
///
/// - [`heap::init`]
pub fn init() {
    heap::init();
    println!("mod memory initialized");
}

// END
