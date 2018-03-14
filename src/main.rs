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

#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![feature(try_trait)]
#![no_std]
#![no_main]

extern crate compiler_builtins;
#[macro_use]
extern crate fixedvec;

#[macro_use]
mod print;
mod uefi;

pub(crate) static mut UEFI_SYSTEM_TABLE: Option<&'static uefi::SystemTable> = None;

#[no_mangle]
pub extern "win64" fn UefiMain(handle: uefi::Handle, system_table: &'static uefi::SystemTable) -> uefi::Status {
    unsafe {
        UEFI_SYSTEM_TABLE = Some(&system_table);
    }
    println!("UEFI header: {:#?}", system_table.get_header());
    main();
    uefi::Status::Success
}

fn main() {
    println!("Hello, {}!", "UEFI world");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("Authors:");
    for author in env!("CARGO_PKG_AUTHORS").split(';') {
        println!("    {}", author);
    }
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(message: core::fmt::Arguments, file: &'static str, line: u32, column: u32) -> ! {
    println!("Panic in {file} at {line}:{column}: {message}", message=message, file=file, line=line, column=column);
    loop {}
}
