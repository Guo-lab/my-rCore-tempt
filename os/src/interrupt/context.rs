use riscv::register::{sstatus::sStatus, scause::Scause};


// *****************************************************************************
// `[repr(C)]` 属性
// 要求 struct 按照 C 语言的规则进行内存分布，否则 Rust 可能按照其他规则进行内存排布
//
#[repr(C)]
#[derive(Debug)]
pub struct Context {
    pub x: [usize; 32],  // 32 个通用寄存器
    //
    /// 保存诸多状态位的特权态寄存器
    pub sstatus: Sstatus,
    //
    /// 保存中断地址的 特权态寄存器
    pub sepc: usize,
}
//
// 使用了rCore 中的库 riscv 封装的寄存器操作
// os/Cargo.toml 中添加依赖 @
//
// ******************************************************************************
 
