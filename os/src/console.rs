// 实现控制台 char 的出入
// 格式化输出
//
// ['core::fmt::Write'] trait 包含 
// 1）须实现的['Write_str']方法和 2）自带但依赖于['Write_str'] ['Write_fmt']的方法
//
// 声明一个类型 为实现['Write_str']后就可以['Write_fmt']来格式化输出
//                        |                     |
//                        V                     V
//          core::fmt::Write::write_str     core::fmt::Write::write_fmt
//
//
// *****************************************************************************  
//  
use crate::sbi::* ;
use core::fmt::{self, Write};

// *****************************************************************************
// ZeroSizedType 只有一个值即为空（本身就是一个单件）
//
struct Stdout;

impl Write for Stdout{
    // 打印一个string
    // sbi每接受一个'usize',都会把它作为'u8'打印字符
    // 有非 ASCII 字符, utf-8, 每一个'u8'调用一次
    //
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let mut buffer = [0u8; 4];
        for c in s.chars() {
            for code_point in c.encode_utf8(&mut buffer).as_bytes().iter() {
                console_putchar(*code_point as usize);
            }
        }
        Ok(())
    }
}
// ***************************************************************************** 


//
// 打印 [`core::format_args!`] 格式化后的数据
// https://doc.rust-lang.org/nightly/core/macro.format_args.html
// [`print!`] and [`println!`] marcos_(宏)将展开成此函数

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();  // 打开
}
//
//
//
// *****************************************************************************
//
// `print!`宏 正如std标准库版
// [`core::fmt::Write`] trait 实现后的 [`console::Stdout`]
//
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}
//
//
//
//
// Marco `println!` 
// [`core::fmt::Write`] trait 实现后的 [`console::Stdout`]
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
        // 添加换行符
    }
}
//
//
// *****************************************************************************
//
