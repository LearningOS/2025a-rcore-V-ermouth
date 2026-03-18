//! SBI console driver, for text output
use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;

/// 实现Write trait，才能支持格式化输出
impl Write for Stdout {
    
    /// 字符输出，调用SBI putchar输出格式化后的字符
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

/// 格式化输出函数
pub fn print_fmt(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// 编写宏定义
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print_fmt(format_args!($fmt $(, $($arg)+)?))
    }
}

/// 实现println宏，自动换行
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print_fmt(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    }
}