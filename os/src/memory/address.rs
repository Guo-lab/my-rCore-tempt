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
//! .......................................................
//! ### 虚拟地址 `VirtualAddress`
//! ```rust
//! /// 通过地址得到任何类型变量的引用。没有类型检查所以要格外注意
//! pub fn deref<T>(self) -> &'static mut T { ... }
//! /// 得到其页内偏移，即低 12 位
//! pub fn page_offset(self) -> usize { ... }
//! ```
//! .......................................................
//! ### 虚拟页号 `VirtualPageNumber`
//! ```rust
//! /// 通过地址得到页面所对应的一段内存
//! pub fn deref(self) -> &'static mut [u8; PAGE_SIZE] { ... }
//! /// 得到一至三级页号
//! pub fn levels(self) -> [usize; 3] { ... }
//! ```
//! .......................................................
//! ### 物理页号 `PhysicalPageNumber`
//! ```rust
//! /// 按照内核线性映射后得到页面对应的一段内存
//! pub fn deref_kernel(self) -> &'static mut [u8; PAGE_SIZE] { ... }
//! ```
//! ********************************************************



//! # 基本运算-地址
//! ********************************************************
//! - 四种类型都可以直接与 `usize` 进行加减，返回结果为原本类型
//! - 四种类型都可以与自己类型进行加减，返回结果为 `usize`
//! ********************************************************


// 2021-3-11
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PhysicalAddress(pub usize);
// *************************************************************************


// 2021-3-12 为了后面方便 余下三种类型地址一并实现
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PhysicalPageNumber(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VirtualAddress(pub usize);
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VirtualPageNumber(pub usize);







use super::config::PAGE_SIZE;









// 2021-3-13
// 以下是一大堆类型的相互转换、各种琐碎操作

impl PhysicalAddress {
    // 2021-3-13 为完成物理地址页框，此时未涉及虚拟地址映射和转换！
    /// 取得页内偏移
    pub fn page_offset(&self) -> usize {
        self.0 % PAGE_SIZE
    }
}


// 2021-3-13
// 转换页框号，Frame 的完成
macro_rules! implement_address_to_page_number {
    // 这里面的类型转换实现 [`From`] trait，会自动实现相反的 [`Into`] trait
    ($address_type: ty, $page_number_type: ty) => {
        impl From<$page_number_type> for $address_type {
            /// 从页号转换为地址
            fn from(page_number: $page_number_type) -> Self {
                Self(page_number.0 * PAGE_SIZE)
            }
        }
        impl From<$address_type> for $page_number_type {
            /// 从地址转换为页号，直接进行移位操作
            ///
            /// 不允许转换没有对齐的地址，这种情况应当使用 `floor()` 和 `ceil()`
            fn from(address: $address_type) -> Self {
                assert!(address.0 % PAGE_SIZE == 0);
                Self(address.0 / PAGE_SIZE)
            }
        }
        impl $page_number_type {
            /// 将地址转换为页号，向下取整
            pub const fn floor(address: $address_type) -> Self {
                Self(address.0 / PAGE_SIZE)
            }
            /// 将地址转换为页号，向上取整
            pub const fn ceil(address: $address_type) -> Self {
                Self(address.0 / PAGE_SIZE + (address.0 % PAGE_SIZE != 0) as usize)
            }
        }
    };
}
implement_address_to_page_number! {PhysicalAddress, PhysicalPageNumber}






















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
ERROR: ^^^^^^^^^^^^^^^^ rustdoc does not generate documentation for macro invocations */

implement_usize_operations! {PhysicalAddress}
// 2021-3-12
implement_usize_operations! {VirtualAddress}
implement_usize_operations! {PhysicalPageNumber}
implement_usize_operations! {VirtualPageNumber}
