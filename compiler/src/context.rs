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

use std;
use std::mem;
use std::fs::File;
use std::collections::HashMap;

use core::bin_structs::{Header, FuncRef, GlobRef};
use core::bin_reader::BinReader;

use crate::func::Func;
use crate::glob::{Glob, GlobalValue};
use crate::instruction::Instruction;
use crate::out_bin::OutBin;

pub struct Context {
    funcs: HashMap<String, Func>,
    globs: HashMap<String, Glob>,
}

impl Context {
    pub fn new() -> Context {
        Context { funcs: HashMap::new(), globs: HashMap::new() }
    }

    pub fn make_func(&mut self, name: &str, result_count: u8) -> Result<Func, String> {
        if self.funcs.contains_key(name) {
            return Err(String::from(format!("Function with name '{}' already exist", name)))
        }

        match self.funcs.insert(String::from(name), Func::new(String::from(name), result_count)) {
            Some(func) => Ok(func),
            None => Err(String::from("Failed to add new func"))
        }
    }

    pub fn make_func_with_code(&mut self, name: &str, result_count: u8, code: Vec<Instruction>) -> Result<Func, String> {
        if self.funcs.contains_key(name) {
            return Err(String::from(format!("Function with name '{}' already exist", name)))
        }

        match self.funcs.insert(String::from(name), Func::new_with_code(String::from(name), result_count, code)) {
            Some(func) => Ok(func),
            None => Err(String::from("Failed to add new func"))
        }
    }

    pub fn get_func(&self, name: &str) -> Option<&Func>{
        self.funcs.get(name)
    }

    pub fn get_mut_func(&mut self, name: &str) -> Option<&mut Func>{
        self.funcs.get_mut(name)
    }

    pub fn make_glob(&mut self, name: &str) -> Result<Glob, String> {
        if self.globs.contains_key(name) {
            return Err(String::from(format!("Global with name '{}' already exist", name)))
        }

        match self.globs.insert(String::from(name), Glob::new(String::from(name))) {
            Some(glob) => Ok(glob),
            None => Err(String::from("Failed to add new glob"))
        }
    }

    pub fn make_glob_with_value(&mut self, name: &str, value: GlobalValue) -> Result<Glob, String> {
        if self.globs.contains_key(name) {
            return Err(String::from(format!("Global with name '{}' already exist", name)))
        }

        match self.globs.insert(String::from(name), Glob::new_with_value(String::from(name), value)) {
            Some(glob) => Ok(glob),
            None => Err(String::from("Failed to add new glob"))
        }
    }

    pub fn get_glob(&self, name: &str) -> Option<&Glob>{
        self.globs.get(name)
    }

    pub fn get_mut_glob(&mut self, name: &str) -> Option<&mut Glob>{
        self.globs.get_mut(name)
    }

    pub fn write_binary(&self, file_name: &str) -> Result<(), String> {
        let mut bin_file = match File::create(file_name) {
            Ok(bin_file) => bin_file,
            Err(error) => return Err(String::from(format!("Failed to create output file\n{}", error.to_string())))
        };
        let mut bin = OutBin::new();

        for func in self.funcs.values() {
            bin.add_func(func);
        }

        for glob in self.globs.values() {
            bin.add_glob(glob);
        }

        for func in self.funcs.values() {
            func.encode(&mut bin)?;
        }

        for glob in self.globs.values() {
            glob.encode(&mut bin)?;
        }

        bin.write(&mut bin_file)
    }

    pub fn load_binary(&self, file_name: &str) -> Result<(), String> {
        let bin_file = match File::open(file_name) {
            Ok(bin_file) => bin_file,
            Err(error) => return Err(String::from(format!("Failed to open binary file\n{}", error.to_string())))
        };

        let mut bin_reader = BinReader::new(bin_file);

        let header = match Header::from_stream(&mut bin_reader) {
            Ok(header) => header,
            Err(error) => return Err(String::from(format!("Failed to parse binary header\n{}", error)))
        };

        let mut str_table = vec![0; header.str_tab_size as usize];
        bin_reader.read_bytes(&mut str_table)?;

        let mut func_refs = vec![];
        let func_count = header.func_tab_size / mem::size_of::<FuncRef>() as u32;
        for _func_ref_idx in 0..func_count {
            let func_ref = match FuncRef::from_stream(&mut bin_reader) {
                Ok(func_ref) => func_ref,
                Err(error) => return Err(String::from(format!("Failed to parse func ref\n{}", error)))
            };
            func_refs.push(func_ref);
        }
        assert!(func_refs.len() == func_count as usize);

        let mut glob_refs = vec![];
        let glob_count = header.glob_tab_size / mem::size_of::<GlobRef>() as u32;
        for _glob_ref_idx in 0..glob_count {
            let glob_ref = match GlobRef::from_stream(&mut bin_reader) {
                Ok(glob_ref) => glob_ref,
                Err(error) => return Err(String::from(format!("Failed to parse glob ref\n{}", error)))
            };
            glob_refs.push(glob_ref);
        }
        assert!(glob_refs.len() == glob_count as usize);

        let mut code = vec![0; header.code_size as usize];
        bin_reader.read_bytes(&mut code)?;

        let mut glob_data = vec![0; header.glob_size as usize];
        bin_reader.read_bytes(&mut glob_data)?;

        for _func_ref in func_refs.iter() {
            // TODO parse string as a name for this function
        }

        for _glob_ref in glob_refs.iter() {
            // TODO parse string as a name for this function
        }

        Ok(())
    }
}