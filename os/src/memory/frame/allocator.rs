//! 提供帧分配器 [`FRAME_ALLOCATOR`](FrameAllocator)
//!
//! 返回的 [`FrameTracker`] 类型代表一个帧，它在被 drop 时会自动将空间补回分配器中。

// 有关具体算法 用名为 Allocator 的 Rust trait 封装起来 /algorithm/src/allocator/mod.rs
// 2021-3-12

// ?! super 2021-3-12
use super::*;
use crate::memory::*;
// 用到了 algorithm
// after cargo new --lib algorithm
// it is not only a crate
use algorithm::*;
use lazy_static::*;
// 对 static mut 修改unsafe 线程冲突 使用lock() 需要添加依赖
use spin::Mutex;







// 2021-3-12

lazy_static! {
    /// 帧分配器 由 lazy 和 mutex 包装
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocator<AllocatorImpl>> = Mutex::new(FrameAllocator::new(Range::from(
              PhysicalPageNumber::ceil(PhysicalAddress::from(*KERNEL_END_ADDRESS))..PhysicalPageNumber::floor(MEMORY_END_ADDRESS),
        )
    ));
}



// *****************************************************************************
/// 基于线段树的帧分配 / 回收
// 分配器会以 PhysicalPageNumber 的 Range 初始化 记录起始地址，回馈区间长度给分配器算法
pub struct FrameAllocator<T: Allocator> {
    /// 可用区间的起始
    start_ppn: PhysicalPageNumber,
    /// 分配器
    allocator: T,
}


// ******************************************************************************
impl<T: Allocator> FrameAllocator<T> {
    /// 创建对象
    pub fn new(range: impl Into<Range<PhysicalPageNumber>> + Copy) -> Self {
        FrameAllocator {
            start_ppn: range.into().start,
            allocator: T::new(range.into().len()),
        }
    }
    /// 分配帧，如果没有剩余则返回 `Err`
    pub fn alloc(&mut self) -> MemoryResult<FrameTracker> {
        self.allocator
            .alloc()
            .ok_or("no available frame to allocate")
            // 取得区间中可用地址作 OFFSET 加上起始地址返回
            .map(|offset| FrameTracker(self.start_ppn + offset))
    }
    // 将被释放的帧添加到空闲列表的尾部
    // 这个函数会在 [`FrameTracker`] 被 drop 时自动调用，不应在其他地方调用
    // 使用了 super
    pub(super) fn dealloc(&mut self, frame: &FrameTracker) {
        self.allocator.dealloc(frame.page_number() - self.start_ppn);
    }
}




// END
