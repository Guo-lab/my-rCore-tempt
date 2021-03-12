//! 定义地址类型和地址常量
//! 我们为虚拟地址和物理地址分别设立类型，利用编译器检查来防止混淆。
//!
//! ************************************************************
//! # 类型
//! - 虚拟地址 [`VirtualAddress`]
//! - 物理地址 [`PhysicalAddress`]
//! - 虚拟页号 [`VirtualPageNumber`]
//! - 物理页号 [`PhysicalPageNumber`]
//! 四种类型均由一个 `usize` 来表示

//! *******************************************************
//! ### 虚拟 ↔ 物理
//! - **只能用于线性映射**，可以使用 `from` 或 `into` 来转换

//! *******************************************************
//! ### 物理地址 `PhysicalAddress`
//! ```rust
//! /// 按照内核线性映射后，得到变量引用
//! pub fn deref_kernel<T>(self) -> &'static mut T { ... }
//! /// 得到其页内偏移，即低 12 位
//! pub fn page_offset(self) -> usize { ... }
//! ```
//! ********************************************************
//! # 基本运算-地址
//! ********************************************************
//! - 四种类型都可以直接与 `usize` 进行加减，返回结果为原本类型
//! - 四种类型都可以与自己类型进行加减，返回结果为 `usize`
//! ********************************************************



#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PhysicalAddress(pub usize);
// *************************************************************************

//
//
//
//
/// ********************************************************************************************
/// 下面这些以后可能会删掉一些
/// 为各种仅包含一个 usize 的类型实现运算操作
macro_rules! implement_usize_operations {
    ($type_name: ty) => {
/// ***************************************************
        /// `+`
        impl core::ops::Add<usize> for $type_name {
            type Output = Self;
            fn add(self, other: usize) -> Self::Output {
                Self(self.0 + other)
            }
        }
        /// `-` 该减法针对 usize 类型
        impl core::ops::Sub<usize> for $type_name {
            type Output = Self;
            fn sub(self, other: usize) -> Self::Output {
                Self(self.0 - other)
            }
        }
        /// `-` 该减法针对地址类型
        impl core::ops::Sub<$type_name> for $type_name {
            type Output = usize;
            fn sub(self, other: $type_name) -> Self::Output {
                self.0 - other.0
            }
        }   
/// *****************************************************
        /// `+=`
        impl core::ops::AddAssign<usize> for $type_name {
            fn add_assign(&mut self, rhs: usize) {
                self.0 += rhs;
            }
        }
        /// `-=`
        impl core::ops::SubAssign<usize> for $type_name {
            fn sub_assign(&mut self, rhs: usize) {
                self.0 -= rhs;
            }
        }
/// *****************************************************
        /// 和 usize 相互转换
        impl From<usize> for $type_name {
            fn from(value: usize) -> Self {
                Self(value)
            }
        }
        /// 和 usize 相互转换
        impl From<$type_name> for usize {
            fn from(value: $type_name) -> Self {
                value.0
            }
        }
/// *****************************************************
        /// 是否有效（0 为无效）
        impl $type_name {   
            pub fn valid(&self) -> bool {
                self.0 != 0
            }
        }
/// *****************************************************
        /// {} 输出
        impl core::fmt::Display for $type_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}(0x{:x})", stringify!($type_name), self.0)
            }
        }
    };
}
//
// 代入地址类型
/* /// 代入地址类型
        | ^^^^^^^^^^^^^^^^ rustdoc does not generate documentation for macro invocations */
implement_usize_operations! {PhysicalAddress}
