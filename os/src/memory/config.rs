/// 操作系统动态分配内存所用的堆大小（8M）
pub const KERNEL_HEAP_SIZE: usize = 0x80_0000;
// 调用Buddy System Allocator 须 Cargo/toml 添加依赖
