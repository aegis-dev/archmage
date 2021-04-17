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

use std::collections::HashMap;
use core::bin_structs::{FuncRef, GlobRef, Header};
use crate::func::Func;
use crate::glob::Glob;
use std::fs::File;
use std::mem;
use std::io::Write;

pub struct OutBin {
    string_table: Vec<u8>,
    string_table_map: HashMap<String, u32>,
    funcs_table: Vec<FuncRef>,
    funcs_table_map: HashMap<String, u32>,
    funcs_code: Vec<u8>,
    globals_table: Vec<GlobRef>,
    globals_table_map: HashMap<String, u32>,
    globals_data: Vec<u8>,
}

impl OutBin {
    pub fn new() -> OutBin {
        OutBin {
            string_table: vec![],
            string_table_map: HashMap::new(),
            funcs_table: vec![],
            funcs_table_map: HashMap::new(),
            funcs_code: vec![],
            globals_table: vec![],
            globals_table_map: HashMap::new(),
            globals_data: vec![],

        }
    }

    pub fn write(&self, file: &mut File) -> Result<(), String> {
        let mut header = Header::new();
        header.str_tab_offset = header.header_size as u64;
        header.str_tab_size = self.string_table.len() as u32;
        header.func_tab_offset = header.str_tab_offset + header.str_tab_size as u64;
        header.func_tab_size = (self.funcs_table.len() * mem::size_of::<FuncRef>()) as u32;
        header.glob_tab_offset = header.func_tab_offset + header.func_tab_size as u64;
        header.glob_tab_size = (self.globals_table.len() * mem::size_of::<GlobRef>()) as u32;
        header.code_offset = header.glob_tab_offset + header.glob_tab_size as u64;
        header.code_size = self.funcs_code.len() as u32;
        header.glob_offset = header.code_offset + header.code_size as u64;
        header.glob_size = self.globals_data.len() as u32;
        header.file_size = header.glob_offset + header.glob_size as u64;
        header.checksum = 0; // TODO: update checksum

        unsafe {
            OutBin::write_bytes(file, OutBin::as_u8_slice(&header))?;
        }
        OutBin::write_bytes(file, &self.string_table)?;
        unsafe {
            for func_ref in self.funcs_table.iter() {
                OutBin::write_bytes(file, OutBin::as_u8_slice(&func_ref))?;
            }
            for glob_ref in self.globals_table.iter() {
                OutBin::write_bytes(file, OutBin::as_u8_slice(&glob_ref))?;
            }
        }
        OutBin::write_bytes(file, &self.funcs_code)?;
        OutBin::write_bytes(file, &self.globals_data)?;

        Ok(())
    }

    pub fn add_func(&mut self, func: &Func) -> u32 {
        match self.funcs_table_map.get(func.get_name()) {
            None => {
                let func_idx = self.funcs_table.len() as u32;
                let func_name_idx = self.add_string(func.get_name());
                self.funcs_table.push(FuncRef::new(func_name_idx));
                self.funcs_table_map.insert(func.get_name().clone(), func_idx);
                func_idx
            }
            Some(func_idx) => *func_idx
        }
    }

    pub fn add_func_code(&mut self, code: &Vec<u8>) -> u64 {
        let func_code_offset = self.funcs_code.len() as u64;
        self.funcs_code.extend_from_slice(code);
        func_code_offset
    }

    pub fn get_func_idx(&self, name: &str) -> Option<&u32> {
        self.funcs_table_map.get(name)
    }

    pub fn get_func_ref(&self, name: &str) -> Option<&FuncRef> {
        let func_idx = *self.get_func_idx(name)?;
        self.funcs_table.get(func_idx as usize)
    }

    pub fn get_func_mut_ref(&mut self, name: &str) -> Option<&mut FuncRef> {
        let func_idx = *self.get_func_idx(name)?;
        self.funcs_table.get_mut(func_idx as usize)
    }

    pub fn add_glob(&mut self, glob: &Glob) -> u32 {
        match self.globals_table_map.get(glob.get_name()) {
            None => {
                let global_idx = self.globals_table.len() as u32;
                let global_name_idx = self.add_string(glob.get_name());
                self.globals_table.push(GlobRef::new(global_name_idx));
                self.globals_table_map.insert(glob.get_name().clone(), global_idx);
                global_idx
            }
            Some(global_idx) => *global_idx
        }
    }

    pub fn get_glob_idx(&self, name: &str) -> Option<&u32> {
        self.globals_table_map.get(name)
    }

    pub fn add_glob_data(&mut self, glob_data: &Vec<u8>) -> u64 {
        let global_data_offset = self.globals_data.len() as u64;
        self.globals_data.extend_from_slice(glob_data);
        global_data_offset
    }

    pub fn get_glob_ref(&self, name: &str) -> Option<&GlobRef> {
        let global_idx = *self.get_glob_idx(name)?;
        self.globals_table.get(global_idx as usize)
    }

    pub fn get_glob_mut_ref(&mut self, name: &str) -> Option<&mut GlobRef> {
        let global_idx = *self.get_glob_idx(name)?;
        self.globals_table.get_mut(global_idx as usize)
    }

    unsafe fn as_u8_slice<T: Sized>(p: &T) -> &[u8] {
        std::slice::from_raw_parts((p as *const T) as *const u8, std::mem::size_of::<T>(),)
    }

    fn write_bytes(file: &mut File, bytes: &[u8]) -> Result<(), String> {
        match file.write(bytes) {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Failed to write bytes"))
        }
    }

    fn add_string(&mut self, value: &str) -> u32 {
        match self.string_table_map.get(value) {
            None => {
                let bytes = value.as_bytes();
                let string_idx = self.string_table.len() as u32;
                self.string_table.extend_from_slice(bytes);
                self.string_table_map.insert(String::from(value), string_idx);
                string_idx
            }
            Some(string_idx) => *string_idx
        }
    }
}