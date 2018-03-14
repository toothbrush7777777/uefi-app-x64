// Copyright © 2017, Joshua Saraceni and contributors
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     http://www.apache.org/licenses/LICENSE-2.0
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::fmt::{self, Write};

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::write_fmt(format_args!($($arg)*)).unwrap());
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn write_fmt(args: fmt::Arguments) -> fmt::Result {
    use uefi::SimpleTextOutputProtocol as Console;
    Write::write_fmt(unsafe { &mut *(::UEFI_SYSTEM_TABLE.unwrap().get_console_out() as *const Console as *mut Console) }, args)
}
