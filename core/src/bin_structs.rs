//
// Copyright © 2020-2021  Egidijus Lileika
//
// This file is part of Archmage - Fantasy Virtual Machine
//
// Archmage is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Archmage is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Archmage. If not, see <https://www.gnu.org/licenses/>.
//

use std::mem;

#[repr(packed)]
pub struct Header {
    pub magic: [u8; 8],
    pub header_size: u16,
    pub checksum: u32,
    pub file_size: u64,
    pub str_tab_offset: u64,
    pub str_tab_size: u32,
    pub func_tab_offset: u64,
    pub func_tab_size: u32,
    pub glob_tab_offset: u64,
    pub glob_tab_size: u32,
    pub code_offset: u64,
    pub code_size: u32,
    pub glob_offset: u64,
    pub glob_size: u32,
}

#[repr(packed)]
pub struct FuncRef {
    pub name_idx: u32,
    pub offset: u64,
    pub size: u32,
    pub result_count: u8
}

#[repr(packed)]
pub struct GlobRef {
    pub name_idx: u32,
    pub offset: u64,
    pub size: u32,
}

impl Header {
    pub fn new() -> Header {
        Header {
            magic: ['a' as u8, 'r' as u8, 'c' as u8, 'h' as u8, 'm' as u8, 'a' as u8, 'g' as u8, 'e' as u8],
            header_size: mem::size_of::<Header>() as u16,
            checksum: 0,
            file_size: 0,
            str_tab_offset: 0,
            str_tab_size: 0,
            func_tab_offset: 0,
            func_tab_size: 0,
            glob_tab_offset: 0,
            glob_tab_size: 0,
            code_offset: 0,
            code_size: 0,
            glob_offset: 0,
            glob_size: 0
        }
    }
}

impl FuncRef {
    pub fn new(name_idx: u32) -> FuncRef {
        FuncRef { name_idx, offset: 0, size: 0, result_count: 0 }
    }
}

impl GlobRef {
    pub fn new(name_idx: u32) -> GlobRef {
        GlobRef { name_idx, offset: 0, size: 0 }
    }
}