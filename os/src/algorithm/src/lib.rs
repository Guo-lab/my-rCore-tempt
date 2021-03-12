//! #[cfg(test)]
//! mod tests {
//!    #[test]
//!     fn it_works() {
//!        assert_eq!(2 + 2, 4);
//!     }
//! }
//! 初始代码 删除 尕写成我们自己的
//! 一些可能用到，而又不好找库的数据结构
//!
//! 以及有多种实现，会留作业的数据结构
#![no_std]


// 2021-3-12
extern crate alloc;
// 2021-3-12
mod allocator;
pub use allocator::*;
