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

#![allow(dead_code)]

mod stack;

use std::mem;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::convert::TryFrom;

use core::ser_des::SerDes;
use core::bin_utils;
use core::byte_vec_reader::ByteVecReader;
use core::bin_structs::{Header, FuncRef, GlobRef};
use core::opcodes::Opcode;

use crate::stack::Stack;


pub struct VM {
    code: ByteVecReader,
    glob_data: Vec<u8>,
    func_refs: Vec<FuncRef>,
    glob_refs: Vec<GlobRef>,
    func_map: HashMap<String, FuncRef>,
    glob_map: HashMap<String, GlobRef>,
    stack: Stack,
    heap: Vec<u8>,
    callstack: Vec<usize>,
}

impl VM {
    pub fn init_from_file(file_name: &str) -> Result<VM, String> {
        let bin_file = match File::open(file_name) {
            Ok(bin_file) => bin_file,
            Err(error) => return Err(String::from(format!("Failed to open binary file\n{}", error.to_string())))
        };

        VM::init(BufReader::new(bin_file))
    }

    pub fn init(bin_reader: BufReader<File>) -> Result<VM, String>  {
        let mut bin_reader = bin_reader;

        let header = match Header::deserialize(&mut bin_reader) {
            Ok(header) => header,
            Err(error) => return Err(String::from(format!("Failed to parse binary header\n{}", error)))
        };

        let mut str_table = vec![0; header.str_tab_size as usize];
        bin_utils::read_bytes(&mut bin_reader, &mut str_table)?;

        let mut func_refs = vec![];
        let mut func_map = HashMap::new();
        let func_count = header.func_tab_size / mem::size_of::<FuncRef>() as u32;
        for _func_ref_idx in 0..func_count {
            let func_ref = match FuncRef::deserialize(&mut bin_reader) {
                Ok(func_ref) => func_ref,
                Err(error) => return Err(String::from(format!("Failed to parse func ref\n{}", error)))
            };
            func_refs.push(func_ref);

            let func_name = bin_utils::str_from_table(&str_table, func_ref.name_idx)?;
            func_map.insert(func_name, func_ref);
        }
        assert!(func_refs.len() == func_count as usize);

        let mut glob_refs = vec![];
        let mut glob_map = HashMap::new();
        let glob_count = header.glob_tab_size / mem::size_of::<GlobRef>() as u32;
        for _glob_ref_idx in 0..glob_count {
            let glob_ref = match GlobRef::deserialize(&mut bin_reader) {
                Ok(glob_ref) => glob_ref,
                Err(error) => return Err(String::from(format!("Failed to parse glob ref\n{}", error)))
            };
            glob_refs.push(glob_ref);

            let glob_name = bin_utils::str_from_table(&str_table, glob_ref.name_idx)?;
            glob_map.insert(glob_name, glob_ref);
        }
        assert!(glob_refs.len() == glob_count as usize);

        let mut code = vec![0; header.code_size as usize];
        bin_utils::read_bytes(&mut bin_reader, &mut code)?;

        let mut glob_data = vec![0; header.glob_size as usize];
        bin_utils::read_bytes(&mut bin_reader, &mut glob_data)?;

        Ok(
            VM {
                code: ByteVecReader::new(code),
                glob_data,
                func_refs,
                glob_refs,
                func_map,
                glob_map,
                stack: Stack::new(),
                heap: vec![],
                callstack: vec![]
            }
        )
    }

    pub fn execute(&mut self, func_name: &str) -> Result<(), String> {
        let func_ref = match self.func_map.get(func_name) {
            Some(func_ref) => func_ref.clone(),
            None => return Err(String::from(format!("Unable to find func '{}'", func_name)))
        };

        self.execute_func(func_ref)
    }

    fn execute_func(&mut self, func_ref: FuncRef) -> Result<(), String> {
        self.callstack.push(self.code.get_current_byte_idx());
        self.code.set_current_byte_idx(func_ref.offset as usize);
        loop {
            let opcode_byte = self.code.read_u8()?;
            let opcode = match Opcode::try_from(opcode_byte) {
                Ok(opcode) => opcode,
                Err(_) => return Err(String::from(format!("Unexpected opcode '{:X}'", opcode_byte)))
            };

            match opcode {
                Opcode::Nop => { }
                Opcode::Return => {
                    match self.callstack.pop() {
                        Some(return_address) => self.code.set_current_byte_idx(return_address),
                        None => {}
                    };
                    return Ok(());
                }
                Opcode::Call => {
                    let func_idx = self.code.read_u32()?;
                    let func_ref = match self.func_refs.get(func_idx as usize) {
                        Some(func_ref) => func_ref.clone(),
                        None => return Err(String::from(format!("Unexpected func idx '{:X}'", func_idx)))
                    };

                    self.execute_func(func_ref)?
                }
                Opcode::Jump => {
                    let offset = self.code.read_u64()?;
                    let program_counter = self.code.get_current_byte_idx();
                    self.code.set_current_byte_idx(program_counter + offset as usize);
                }
                Opcode::JumpC => {
                    let offset = self.stack.pop()?;
                    let program_counter = self.code.get_current_byte_idx();
                    self.code.set_current_byte_idx(program_counter + offset as usize);
                }
                Opcode::Pop => {
                    self.stack.pop()?;
                }
                Opcode::StackGet => {
                    let stack_offset = self.code.read_u64()?;
                    self.stack.push(self.stack.get(stack_offset)?)
                }
                Opcode::StackSet => {
                    let value = self.stack.get(0)?;
                    let stack_offset = self.code.read_u64()?;
                    self.stack.push(self.stack.set(stack_offset, value)?)
                }
                Opcode::I64Const => {
                    let value = self.code.read_u64()?;
                    self.stack.push(value);
                }
                Opcode::F64Const => {
                    let value = self.code.read_u64()?;
                    self.stack.push(value);
                }
                Opcode::I8Load => {}
                Opcode::I8LoadC => {}
                Opcode::I16Load => {}
                Opcode::I16LoadC => {}
                Opcode::I32Load => {}
                Opcode::I32LoadC => {}
                Opcode::I64Load => {}
                Opcode::I64LoadC => {}
                Opcode::F32Load => {}
                Opcode::F32LoadC => {}
                Opcode::F64Load => {}
                Opcode::F64LoadC => {}
                Opcode::I8Store => {}
                Opcode::I8StoreC => {}
                Opcode::I16Store => {}
                Opcode::I16StoreC => {}
                Opcode::I32Store => {}
                Opcode::I32StoreC => {}
                Opcode::I64Store => {}
                Opcode::I64StoreC => {}
                Opcode::F32Store => {}
                Opcode::F32StoreC => {}
                Opcode::F64Store => {}
                Opcode::F64StoreC => {}
                Opcode::I64Eqz => {}
                Opcode::I64Eq => {}
                Opcode::I64Ne => {}
                Opcode::I64LtS => {}
                Opcode::I64LtU => {}
                Opcode::I64GtS => {}
                Opcode::I64GtU => {}
                Opcode::I64LeS => {}
                Opcode::I64LeU => {}
                Opcode::I64GeS => {}
                Opcode::I64GeU => {}
                Opcode::F64Eq => {}
                Opcode::F64Ne => {}
                Opcode::F64Lt => {}
                Opcode::F64Gt => {}
                Opcode::F64Le => {}
                Opcode::F64Ge => {}
                Opcode::I64Add => {}
                Opcode::I64Sub => {}
                Opcode::I64Mul => {}
                Opcode::I64DivS => {}
                Opcode::I64DivU => {}
                Opcode::I64RemS => {}
                Opcode::I64RemU => {}
                Opcode::I64Pow => {}
                Opcode::I64Abs => {}
                Opcode::I64Sqrt => {}
                Opcode::I64And => {}
                Opcode::I64Or => {}
                Opcode::I64Xor => {}
                Opcode::I64Shl => {}
                Opcode::I64ShrS => {}
                Opcode::I64ShrU => {}
                Opcode::I64Rotl => {}
                Opcode::I64Rotr => {}
                Opcode::F64Add => {}
                Opcode::F64Sub => {}
                Opcode::F64Mul => {}
                Opcode::F64Div => {}
                Opcode::F64Pow => {}
                Opcode::F64Abs => {}
                Opcode::F64Ceil => {}
                Opcode::F64Floor => {}
                Opcode::F64Trunc => {}
                Opcode::F64Nearest => {}
                Opcode::F64Sqrt => {}
                unknown => {
                    return Err(String::from(format!("Unimplemented instruction '{:X}'", unknown as u8)))
                }
            }
        }
    }
}