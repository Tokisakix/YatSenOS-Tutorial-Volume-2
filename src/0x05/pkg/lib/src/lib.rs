<<<<<<< HEAD
#![allow(dead_code)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![cfg_attr(not(test), no_std)]
=======
#![cfg_attr(not(test), no_std)]
#![allow(dead_code, unused_imports)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4

#[macro_use]
pub mod macros;

#[macro_use]
extern crate syscall_def;

#[macro_use]
pub mod io;
pub mod allocator;
pub extern crate alloc;

mod syscall;
<<<<<<< HEAD
mod utils;
=======
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4

use core::fmt::*;

pub use alloc::*;
<<<<<<< HEAD
pub use chrono::*;
pub use io::*;
pub use syscall::*;
pub use utils::*;
=======
pub use io::*;
pub use syscall::*;
>>>>>>> 5e6e567754b757eb2bb7dc4d28e2a848efc12ef4

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => ($crate::_err(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! errln {
    () => ($crate::err!("\n"));
    ($($arg:tt)*) => ($crate::err!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
    stdout().write(format!("{}", args).as_str());
}

#[doc(hidden)]
pub fn _err(args: Arguments) {
    stderr().write(format!("{}", args).as_str());
}
