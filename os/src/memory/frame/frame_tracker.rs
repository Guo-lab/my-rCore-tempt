//! 提供物理页的「`Box`」 [`FrameTracker`]

/// 分配出的物理页
///
/// # `Tracker` 是什么？
/// > 可以理解为 [`Box`](alloc::boxed::Box)，而区别在于，其空间不是分配在堆上，
/// > 而是直接在内存中划一片（一个物理页）。
///
/// 在我们实现操作系统的过程中，会经常遇到「指定一块内存区域作为某种用处」的情况。
/// 此时，我们说这块内存可以用，但是因为它不在堆栈上，Rust 编译器并不知道它是什么，所以
/// 我们需要 unsafe 地将其转换为 `&'static mut T` 的形式（`'static` 一般可以省略）。
///
/// 但是，比如我们用一块内存来作为页表，而当这个页表我们不再需要的时候，就应当释放空间。
/// 我们其实更需要一个像「创建一个有生命期的对象」一样的模式来使用这块内存 [页表] 。因此，
/// 我们不妨用 `Tracker` 类型来封装这样一个 `&'static mut` 引用。
///
/// 使用 `Tracker` 其实就很像使用一个 smart pointer。如果需要引用计数，
/// 就在外面再套一层 [`Arc`](alloc::sync::Arc) 就好
pub struct FrameTracker(pub(super) PhysicalPageNumber);


// 2021-3-12
use crate::memory::{address::*, FRAME_ALLOCATOR};
use crate::memory::PAGE_SIZE;



//
// 2021-3-12  实现 FrameTracker 这个结构
// 分配器分配给 FrameTracker 作为一个帧的 biao 示
// 析构的时候 Drop 回收
impl FrameTracker {
    /// 帧的物理地址
    pub fn address(&self) -> PhysicalAddress {
        self.0.into()
    }
    /// 帧的物理页号
    pub fn page_number(&self) -> PhysicalPageNumber {
        self.0
    }
}


/// 帧在释放时会放回 [`static@FRAME_ALLOCATOR`] 的空闲链表中
impl Drop for FrameTracker {
    fn drop(&mut self) {
        FRAME_ALLOCATOR.lock().dealloc(self);
    }
}









// 2021-3-15 when dereference
/*
error[E0614]: type `frame_tracker::FrameTracker` cannot be dereferenced
   --> src/memory/mapping/mapping.rs:143:21
    |
143 |                     (*frame).copy_from_slice(&page_data);
    |                     ^^^^^^^^
*/
// So .......
/// `FrameTracker` 可以 deref 得到对应的 `[u8; PAGE_SIZE]`
impl core::ops::Deref for FrameTracker {
    type Target = [u8; PAGE_SIZE];
    fn deref(&self) -> &Self::Target {
        self.page_number().deref_kernel()
    }
}

/// `FrameTracker` 可以 deref 得到对应的 `[u8; PAGE_SIZE]`
impl core::ops::DerefMut for FrameTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.page_number().deref_kernel()
    }
}

// And Then
//  --> src/memory/frame/frame_tracker.rs:78:28
//   |
//78 |  self.page_number().deref_kernel()
//   |                     ^^^^^^^^^^^^ method not found in `address::PhysicalPageNumber`
// So enter /memory/address.rs





// END




