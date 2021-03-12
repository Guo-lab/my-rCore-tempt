//! 负责分配 / 回收的数据结构




// 2021-3-12
/// 分配器：固定容量，每次分配 / 回收一个元素
pub trait Allocator {
    /// 给定容量，创建分配器
    fn new(capacity: usize) -> Self;
    /// 分配一个元素，无法分配则返回 `None`
    fn alloc(&mut self) -> Option<usize>;
    /// 回收一个元素
    fn dealloc(&mut self, index: usize);
}


// 栈式分配和线段树分配 algorithm/src/allocator
mod stacked_allocator;
mod segment_tree_allocator;
pub use stacked_allocator::StackedAllocator;
pub use segment_tree_allocator::SegmentTreeAllocator;

/// 默认使用的分配器
//pub type AllocatorImpl = SegmentTreeAllocator;
// pub type AllocatorImpl = StackedAllocator;
pub type AllocatorImpl = SegmentTreeAllocator;







// END
