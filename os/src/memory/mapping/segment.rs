//! 线形映射出现在KNRNEL
//! 不能全部线性,由此基于Page分配出现用户
//! enum and struct 封装内存段映射的类型和其本身
//! 映射类型 [`MapType`] 和映射片段 [`Segment`]

use crate::memory::{address::*, mapping::Flags, range::Range};


// **********************************************************
/// 映射的类型
//#[derive(Debug)]
//  23 |     pub map_type: MapType,
//     |     --------------------- this field does not implement `Copy`
// ERROR SOLUTION:
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MapType {
    /// 线性映射，操作系统使用
    Linear,
    /// 按帧分配映射
    Framed,
}

/// 一个映射片段（对应旧 tutorial 的 `MemoryArea`）
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub struct Segment {
    /// 映射类型
    pub map_type: MapType,
    /// 所映射的虚拟地址
    pub range: Range<VirtualAddress>,
    /// 权限标志
    pub flags: Flags,
}



// *******************************************************
// 上层须要把一个Segment没建立起物理也映射关系的全部虚拟Page申请到物理Page并映射
impl Segment {
    /// 遍历对应的物理地址（如果可能）
    pub fn iter_mapped(&self) -> Option<impl Iterator<Item = PhysicalPageNumber>> {
        match self.map_type {
            // 线性映射可以直接将虚拟地址转换
    // 2021-3-15
    // page_range function follow [pub fn page_range()] 
   
    // 2021-3-15 range.rs
/*--> src/memory/mapping/segment.rs:44:55
   |
44 |             MapType::Linear => Some(self.page_range().into().iter()), 
   |                                     ------------------^^^^--
   |                                     |                 |
   |                                     |                 cannot infer type for type parameter `T` declared on the trait `Into`
   |                                     this method call resolves to `T`
   |
   = note: type must be known at this point
*/  // into() realize
            MapType::Linear => Some(self.page_range().into().iter()), 


            // 按帧映射无法直接获得物理地址，需要分配
            MapType::Framed => None,
        }
    }

    /// 将地址相应地上下取整，获得虚拟页号区间
    pub fn page_range(&self) -> Range<VirtualPageNumber> {
        Range::from(
            VirtualPageNumber::floor(self.range.start)..VirtualPageNumber::ceil(self.range.end),
        )
    }
}







// End
