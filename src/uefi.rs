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

const MOST_SIGNIFICANT_BIT: usize = usize::max_value() - usize::max_value() / 2;
const STATUS_ERROR_MASK: usize = MOST_SIGNIFICANT_BIT;

/// Pointer to data structure.
pub type Handle = *mut ();

/// Represents an unimplemented part of the UEFI API.
/// Produces warnings when used.
#[deprecated(note = "Define the correct interface")]
pub enum Unimplemented {}
pub type TODO = *const Unimplemented;

#[repr(usize)]
#[derive(Clone, Copy, Debug)]
pub enum Status {
    Success = 0,

    // Warnings
    UnknownGlyph = 1,

    // Errors
    Unsupported = STATUS_ERROR_MASK | 3,
    DeviceError = STATUS_ERROR_MASK | 7,
}

impl ::core::ops::Try for Status {
    type Ok = Self;
    type Error = Self;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            success @ Status::Success => Ok(success),
            error @ _ => Err(error),
        }
    }

    fn from_error(status: Self::Error) -> Self { status }
    fn from_ok(status: Self::Ok) -> Self { status }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    _reserved: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SystemTable {
    header: TableHeader,
    firmware_vendor: *const u16,
    firmware_revision: u32,
    console_in_handle: TODO,
    console_in: TODO,
    console_out_handle: TODO,
    console_out: *const SimpleTextOutputProtocol,
    standard_error_handle: TODO,
    standard_error: TODO,
    runtime_services: TODO,
    boot_services: TODO,
    total_table_entries: usize,
    configuration_table: TODO,
}

impl SystemTable {
    pub fn get_header(&self) -> &TableHeader {
        &self.header
    }
    pub fn get_console_out(&self) -> &'static SimpleTextOutputProtocol {
        unsafe { &*self.console_out }
    }
}

#[repr(C)]
pub struct SimpleTextOutputProtocol {
    pub reset: extern "win64" fn(*const SimpleTextOutputProtocol, extended_verification: bool) -> Status,
    output_string: extern "win64" fn(*const SimpleTextOutputProtocol, string: *const u16) -> Status,
    pub test_string: extern "win64" fn(*const SimpleTextOutputProtocol, string: *const u16) -> Status,
    query_mode: TODO,
    set_mode: TODO,
    pub set_attribute: extern "win64" fn(*const SimpleTextOutputProtocol, attribute: usize) -> Status,
    pub clear_screen: extern "win64" fn(*const SimpleTextOutputProtocol) -> Status,
    pub set_cursor_position: extern "win64" fn(*const SimpleTextOutputProtocol, column: usize, row: usize) -> Status,
    pub enable_cursor: extern "win64" fn(*const SimpleTextOutputProtocol, visible: bool) -> Status,
    mode: TODO,
}

impl SimpleTextOutputProtocol {
    pub fn output_null_terminated_ucs2_str(&self, ucs2_chars: *const u16) -> Status {
        (self.output_string)(self as *const _, ucs2_chars)
    }
}

use core::fmt::{self, Write};

impl SimpleTextOutputProtocol {
    fn print_null_terminated_ucs2(&mut self, ucs2_chars: &[u16]) -> Status {
        self.output_null_terminated_ucs2_str(ucs2_chars as *const [u16] as _)
    }

    pub fn print(&mut self, string: &str) -> Status {
        use fixedvec::{FixedVec, ErrorKind::NoSpace};

        const NULL_CHARACTER: u16 = 0x0000;
        const NULL_TERMINATED_NEWLINE: &[u16] = &[0x000d, 0x000a, NULL_CHARACTER];

        let mut print_new_line = false;

        for line in string.split('\n') {
            if print_new_line {
                self.print_null_terminated_ucs2(NULL_TERMINATED_NEWLINE)?;
            } else {
                print_new_line = true;
            }

            let mut buffer = &mut [NULL_CHARACTER; 256];
            let mut ucs2_chars_vec = FixedVec::new(&mut buffer[0..255]);

            for character in line.chars() {
                if character != '\r' {
                    let mut ucs2_pair_buffer = [0u16; 2];
                    for ucs2_char in character.encode_utf16(&mut ucs2_pair_buffer) {
                        match ucs2_chars_vec.push(*ucs2_char) {
                            Err(NoSpace) => {
                                // Print the string.
                                // Note: The buffer already ends with a null character.
                                self.print_null_terminated_ucs2(ucs2_chars_vec.as_slice())?;

                                ucs2_chars_vec.clear();
                                ucs2_chars_vec.push(*ucs2_char);
                            },
                            _ => (),
                        }
                    }
                }
            }

            // End the string with a null character.
            // The result is ignored because the buffer ends with a null character.
            let _ = ucs2_chars_vec.push(NULL_CHARACTER);
            // Print the string.
            self.print_null_terminated_ucs2(ucs2_chars_vec.as_slice())?;
        }

        Status::Success
    }
}

impl fmt::Write for SimpleTextOutputProtocol {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        match self.print(string) {
            Status::Success => Ok(()),
            _ => Err(fmt::Error),
        }
    }
}
