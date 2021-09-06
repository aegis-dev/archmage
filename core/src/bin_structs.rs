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
use std::io::BufReader;
use std::fs::File;

use crate::bin_utils;
use crate::ser_des::SerDes;

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
#[derive(Clone, Copy)]
pub struct FuncRef {
    pub name_idx: u32,
    pub offset: u64,
    pub size: u32,
    pub result_count: u8
}

#[repr(packed)]
#[derive(Clone, Copy)]
pub struct GlobRef {
    pub name_idx: u32,
    pub offset: u64,
    pub size: u32,
    pub value_type: u8,
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

impl SerDes<Header> for Header {
    fn deserialize(reader: &mut BufReader<File>) -> Result<Header, String> {
        let mut magic: [u8; 8] = [0; 8];
        bin_utils::read_bytes(reader, &mut magic)?;
        Ok(Header {
            magic,
            header_size: bin_utils::read_u16(reader)?,
            checksum: bin_utils::read_u32(reader)?,
            file_size: bin_utils::read_u64(reader)?,
            str_tab_offset: bin_utils::read_u64(reader)?,
            str_tab_size: bin_utils::read_u32(reader)?,
            func_tab_offset: bin_utils::read_u64(reader)?,
            func_tab_size: bin_utils::read_u32(reader)?,
            glob_tab_offset: bin_utils::read_u64(reader)?,
            glob_tab_size: bin_utils::read_u32(reader)?,
            code_offset: bin_utils::read_u64(reader)?,
            code_size: bin_utils::read_u32(reader)?,
            glob_offset: bin_utils::read_u64(reader)?,
            glob_size: bin_utils::read_u32(reader)?,
        })
    }

    fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.magic);
        bytes.extend_from_slice(&self.header_size.to_le_bytes());
        bytes.extend_from_slice(&self.checksum.to_le_bytes());
        bytes.extend_from_slice(&self.file_size.to_le_bytes());
        bytes.extend_from_slice(&self.str_tab_offset.to_le_bytes());
        bytes.extend_from_slice(&self.str_tab_size.to_le_bytes());
        bytes.extend_from_slice(&self.func_tab_offset.to_le_bytes());
        bytes.extend_from_slice(&self.func_tab_size.to_le_bytes());
        bytes.extend_from_slice(&self.glob_tab_offset.to_le_bytes());
        bytes.extend_from_slice(&self.glob_tab_size.to_le_bytes());
        bytes.extend_from_slice(&self.code_offset.to_le_bytes());
        bytes.extend_from_slice(&self.code_size.to_le_bytes());
        bytes.extend_from_slice(&self.glob_offset.to_le_bytes());
        bytes.extend_from_slice(&self.glob_size.to_le_bytes());

        bytes
    }
}

impl FuncRef {
    pub fn new(name_idx: u32) -> FuncRef {
        FuncRef { name_idx, offset: 0, size: 0, result_count: 0 }
    }
}

impl SerDes<FuncRef> for FuncRef {
    fn deserialize(reader: &mut BufReader<File>) -> Result<FuncRef, String> {
        Ok(FuncRef {
            name_idx: bin_utils::read_u32(reader)?,
            offset: bin_utils::read_u64(reader)?,
            size: bin_utils::read_u32(reader)?,
            result_count: bin_utils::read_u8(reader)?
        })
    }

    fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.name_idx.to_le_bytes());
        bytes.extend_from_slice(&self.offset.to_le_bytes());
        bytes.extend_from_slice(&self.size.to_le_bytes());
        bytes.extend_from_slice(&self.result_count.to_le_bytes());

        bytes
    }
}

impl GlobRef {
    pub fn new(name_idx: u32) -> GlobRef {
        GlobRef { name_idx, offset: 0, size: 0, value_type: 0 }
    }
}

impl SerDes<GlobRef> for GlobRef {
    fn deserialize(reader: &mut BufReader<File>) -> Result<GlobRef, String> {
        Ok(GlobRef {
            name_idx: bin_utils::read_u32(reader)?,
            offset: bin_utils::read_u64(reader)?,
            size: bin_utils::read_u32(reader)?,
            value_type: bin_utils::read_u8(reader)?,
        })
    }

    fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend_from_slice(&self.name_idx.to_le_bytes());
        bytes.extend_from_slice(&self.offset.to_le_bytes());
        bytes.extend_from_slice(&self.size.to_le_bytes());
        bytes.extend_from_slice(&self.value_type.to_le_bytes());

        bytes
    }
}