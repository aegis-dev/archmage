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
use core::byte_vec::ByteVec;
use core::bin_structs::{Header, FuncRef, GlobRef};
use core::opcodes::Opcode;

use crate::stack::Stack;


pub struct VM {
    code: ByteVec,
    code_size: usize,
    glob_data: ByteVec,
    glob_data_size: usize,
    func_refs: Vec<FuncRef>,
    glob_refs: Vec<GlobRef>,
    func_map: HashMap<String, FuncRef>,
    glob_map: HashMap<String, GlobRef>,
    stack: Stack,
    heap: ByteVec,
    heap_size: u32,
    callstack: Vec<usize>,
}

impl VM {
    pub fn init_from_file(file_name: &str, heap_size: u32) -> Result<VM, String> {
        let bin_file = match File::open(file_name) {
            Ok(bin_file) => bin_file,
            Err(error) => return Err(String::from(format!("Failed to open binary file\n{}", error.to_string())))
        };

        VM::init(BufReader::new(bin_file), heap_size)
    }

    pub fn init(bin_reader: BufReader<File>, heap_size: u32) -> Result<VM, String>  {
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
                code: ByteVec::new(code),
                code_size: header.code_size as usize,
                glob_data: ByteVec::new(glob_data),
                glob_data_size: header.glob_size as usize,
                func_refs,
                glob_refs,
                func_map,
                glob_map,
                stack: Stack::new(),
                heap: ByteVec::new(vec![0; heap_size as usize]),
                heap_size,
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

                    self.execute_func(func_ref)?;
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
                    let value = self.stack.get(stack_offset)?;
                    self.stack.push(value)?;
                }
                Opcode::StackSet => {
                    let value = self.stack.get(0)?;
                    let stack_offset = self.code.read_u64()?;
                    self.stack.set(stack_offset, value)?;
                }
                Opcode::I64Const => {
                    let value = self.code.read_u64()?;
                    self.stack.push(value)?;
                }
                Opcode::F64Const => {
                    let value = self.code.read_u64()?;
                    self.stack.push(value)?;
                }
                Opcode::I8Load => {
                    let glob_idx = self.code.read_u32()?;
                    let glob_ref = match self.glob_refs.get(glob_idx as usize) {
                        Some(glob_ref) => glob_ref,
                        None => return Err(String::from(format!("Unexpected glob idx '{:X}'", glob_idx)))
                    };

                    self.glob_data.set_current_byte_idx(glob_ref.offset as usize);
                    let value = self.glob_data.read_u8()?;
                    self.stack.push(value as u64)?;
                }
                Opcode::I8LoadC => {
                    let mut address = self.stack.pop()? as usize;
                    let value = match address >= self.glob_data_size {
                        true => {
                            address = address - self.glob_data_size;
                            if address >= self.heap_size as usize {
                                return Err(String::from("Heap overflow"));
                            }
                            self.heap.set_current_byte_idx(address);
                            self.heap.read_u8()?
                        }
                        false => {
                            self.glob_data.set_current_byte_idx(address);
                            self.glob_data.read_u8()?
                        }
                    };

                    self.stack.push(value as u64)?;
                }
                Opcode::I16Load => {
                    let glob_idx = self.code.read_u32()?;
                    let glob_ref = match self.glob_refs.get(glob_idx as usize) {
                        Some(glob_ref) => glob_ref,
                        None => return Err(String::from(format!("Unexpected glob idx '{:X}'", glob_idx)))
                    };

                    self.glob_data.set_current_byte_idx(glob_ref.offset as usize);
                    let value = self.glob_data.read_u16()?;
                    self.stack.push(value as u64)?;
                }
                Opcode::I16LoadC => {
                    let mut address = self.stack.pop()? as usize;
                    let value = match address >= self.glob_data_size {
                        true => {
                            address = address - self.glob_data_size;
                            if address >= self.heap_size as usize {
                                return Err(String::from("Heap overflow"));
                            }
                            self.heap.set_current_byte_idx(address);
                            self.heap.read_u16()?
                        }
                        false => {
                            self.glob_data.set_current_byte_idx(address);
                            self.glob_data.read_u16()?
                        }
                    };

                    self.stack.push(value as u64)?;
                }
                Opcode::I32Load => {
                    let glob_idx = self.code.read_u32()?;
                    let glob_ref = match self.glob_refs.get(glob_idx as usize) {
                        Some(glob_ref) => glob_ref,
                        None => return Err(String::from(format!("Unexpected glob idx '{:X}'", glob_idx)))
                    };

                    self.glob_data.set_current_byte_idx(glob_ref.offset as usize);
                    let value = self.glob_data.read_u32()?;
                    self.stack.push(value as u64)?;
                }
                Opcode::I32LoadC => {
                    let mut address = self.stack.pop()? as usize;
                    let value = match address >= self.glob_data_size {
                        true => {
                            address = address - self.glob_data_size;
                            if address >= self.heap_size as usize {
                                return Err(String::from("Heap overflow"));
                            }
                            self.heap.set_current_byte_idx(address);
                            self.heap.read_u32()?
                        }
                        false => {
                            self.glob_data.set_current_byte_idx(address);
                            self.glob_data.read_u32()?
                        }
                    };

                    self.stack.push(value as u64)?;
                }
                Opcode::I64Load => {
                    let glob_idx = self.code.read_u32()?;
                    let glob_ref = match self.glob_refs.get(glob_idx as usize) {
                        Some(glob_ref) => glob_ref,
                        None => return Err(String::from(format!("Unexpected glob idx '{:X}'", glob_idx)))
                    };

                    self.glob_data.set_current_byte_idx(glob_ref.offset as usize);
                    let value = self.glob_data.read_u64()?;
                    self.stack.push(value as u64)?;
                }
                Opcode::I64LoadC => {
                    let mut address = self.stack.pop()? as usize;
                    let value = match address >= self.glob_data_size {
                        true => {
                            address = address - self.glob_data_size;
                            if address >= self.heap_size as usize {
                                return Err(String::from("Heap overflow"));
                            }
                            self.heap.set_current_byte_idx(address);
                            self.heap.read_u64()?
                        }
                        false => {
                            self.glob_data.set_current_byte_idx(address);
                            self.glob_data.read_u64()?
                        }
                    };

                    self.stack.push(value)?;
                }
                Opcode::F32Load => {
                    let glob_idx = self.code.read_u32()?;
                    let glob_ref = match self.glob_refs.get(glob_idx as usize) {
                        Some(glob_ref) => glob_ref,
                        None => return Err(String::from(format!("Unexpected glob idx '{:X}'", glob_idx)))
                    };

                    self.glob_data.set_current_byte_idx(glob_ref.offset as usize);

                    // Read f32 value and convert it to f64 then cast bytes to u64 and push to stack
                    let value = self.glob_data.read_f32()? as f64;
                    let bytes_as_u64 = u64::from_le_bytes(value.to_le_bytes());
                    self.stack.push(bytes_as_u64)?;
                }
                Opcode::F32LoadC => {
                    let mut address = self.stack.pop()? as usize;
                    let value = match address >= self.glob_data_size {
                        true => {
                            address = address - self.glob_data_size;
                            if address >= self.heap_size as usize {
                                return Err(String::from("Heap overflow"));
                            }
                            self.heap.set_current_byte_idx(address);
                            self.heap.read_f32()?
                        }
                        false => {
                            self.glob_data.set_current_byte_idx(address);
                            self.glob_data.read_f32()?
                        }
                    };

                    // Read f32 value and convert it to f64 then cast bytes to u64 and push to stack
                    let bytes_as_u64 = u64::from_le_bytes((value as f64).to_le_bytes());
                    self.stack.push(bytes_as_u64)?;
                }
                Opcode::F64Load => {
                    let glob_idx = self.code.read_u32()?;
                    let glob_ref = match self.glob_refs.get(glob_idx as usize) {
                        Some(glob_ref) => glob_ref,
                        None => return Err(String::from(format!("Unexpected glob idx '{:X}'", glob_idx)))
                    };

                    self.glob_data.set_current_byte_idx(glob_ref.offset as usize);

                    // Read f64 representation as u64 and push to stack bytes
                    let value = self.glob_data.read_u64()?;
                    self.stack.push(value)?;
                }
                Opcode::F64LoadC => {
                    let mut address = self.stack.pop()? as usize;
                    let value = match address >= self.glob_data_size {
                        true => {
                            address = address - self.glob_data_size;
                            if address >= self.heap_size as usize {
                                return Err(String::from("Heap overflow"));
                            }
                            self.heap.set_current_byte_idx(address);
                            self.heap.read_u64()?
                        }
                        false => {
                            self.glob_data.set_current_byte_idx(address);
                            self.glob_data.read_u64()?
                        }
                    };

                    // Read f64 representation as u64 and push to stack bytes
                    self.stack.push(value)?;
                }
                Opcode::I8Store => {
                    let glob_idx = self.code.read_u32()?;
                    let glob_ref = match self.glob_refs.get(glob_idx as usize) {
                        Some(glob_ref) => glob_ref,
                        None => return Err(String::from(format!("Unexpected glob idx '{:X}'", glob_idx)))
                    };

                    let value = self.stack.pop()?;
                    self.glob_data.set_current_byte_idx(glob_ref.offset as usize);
                    self.glob_data.write_u8(value as u8)?;
                }
                Opcode::I8StoreC => {
                    let mut address = self.stack.pop()? as usize;
                    let value = self.stack.pop()?;
                    match address >= self.glob_data_size {
                        true => {
                            address = address - self.glob_data_size;
                            if address >= self.heap_size as usize {
                                return Err(String::from("Heap overflow"));
                            }
                            self.heap.set_current_byte_idx(address);
                            self.heap.write_u8(value as u8)?;
                        }
                        false => {
                            self.glob_data.set_current_byte_idx(address);
                            self.glob_data.write_u8(value as u8)?;
                        }
                    };
                }
                Opcode::I16Store => {
                    let glob_idx = self.code.read_u32()?;
                    let glob_ref = match self.glob_refs.get(glob_idx as usize) {
                        Some(glob_ref) => glob_ref,
                        None => return Err(String::from(format!("Unexpected glob idx '{:X}'", glob_idx)))
                    };

                    let value = self.stack.pop()?;
                    self.glob_data.set_current_byte_idx(glob_ref.offset as usize);
                    self.glob_data.write_u16(value as u16)?;
                }
                Opcode::I16StoreC => {
                    let mut address = self.stack.pop()? as usize;
                    let value = self.stack.pop()?;
                    match address >= self.glob_data_size {
                        true => {
                            address = address - self.glob_data_size;
                            if address >= self.heap_size as usize {
                                return Err(String::from("Heap overflow"));
                            }
                            self.heap.set_current_byte_idx(address);
                            self.heap.write_u16(value as u16)?;
                        }
                        false => {
                            self.glob_data.set_current_byte_idx(address);
                            self.glob_data.write_u16(value as u16)?;
                        }
                    };
                }
                Opcode::I32Store => {
                    let glob_idx = self.code.read_u32()?;
                    let glob_ref = match self.glob_refs.get(glob_idx as usize) {
                        Some(glob_ref) => glob_ref,
                        None => return Err(String::from(format!("Unexpected glob idx '{:X}'", glob_idx)))
                    };

                    let value = self.stack.pop()?;
                    self.glob_data.set_current_byte_idx(glob_ref.offset as usize);
                    self.glob_data.write_u32(value as u32)?;
                }
                Opcode::I32StoreC => {
                    let mut address = self.stack.pop()? as usize;
                    let value = self.stack.pop()?;
                    match address >= self.glob_data_size {
                        true => {
                            address = address - self.glob_data_size;
                            if address >= self.heap_size as usize {
                                return Err(String::from("Heap overflow"));
                            }
                            self.heap.set_current_byte_idx(address);
                            self.heap.write_u32(value as u32)?;
                        }
                        false => {
                            self.glob_data.set_current_byte_idx(address);
                            self.glob_data.write_u32(value as u32)?;
                        }
                    };
                }
                Opcode::I64Store => {
                    let glob_idx = self.code.read_u32()?;
                    let glob_ref = match self.glob_refs.get(glob_idx as usize) {
                        Some(glob_ref) => glob_ref,
                        None => return Err(String::from(format!("Unexpected glob idx '{:X}'", glob_idx)))
                    };

                    let value = self.stack.pop()?;
                    self.glob_data.set_current_byte_idx(glob_ref.offset as usize);
                    self.glob_data.write_u64(value as u64)?;
                }
                Opcode::I64StoreC => {
                    let mut address = self.stack.pop()? as usize;
                    let value = self.stack.pop()?;
                    match address >= self.glob_data_size {
                        true => {
                            address = address - self.glob_data_size;
                            if address >= self.heap_size as usize {
                                return Err(String::from("Heap overflow"));
                            }
                            self.heap.set_current_byte_idx(address);
                            self.heap.write_u64(value as u64)?;
                        }
                        false => {
                            self.glob_data.set_current_byte_idx(address);
                            self.glob_data.write_u64(value as u64)?;
                        }
                    };
                }
                Opcode::F32Store => {
                    let glob_idx = self.code.read_u32()?;
                    let glob_ref = match self.glob_refs.get(glob_idx as usize) {
                        Some(glob_ref) => glob_ref,
                        None => return Err(String::from(format!("Unexpected glob idx '{:X}'", glob_idx)))
                    };

                    // Read u64 representation of float, convert to f64 and then cast to f32 and write bytes
                    let value = self.stack.pop()?;
                    let float_value = f64::from_le_bytes(value.to_le_bytes());
                    self.glob_data.set_current_byte_idx(glob_ref.offset as usize);
                    self.glob_data.write_f32(float_value as f32)?;
                }
                Opcode::F32StoreC => {
                    let mut address = self.stack.pop()? as usize;
                    let value = self.stack.pop()?;
                    let float_value = f64::from_le_bytes(value.to_le_bytes());

                    match address >= self.glob_data_size {
                        true => {
                            address = address - self.glob_data_size;
                            if address >= self.heap_size as usize {
                                return Err(String::from("Heap overflow"));
                            }
                            self.heap.set_current_byte_idx(address);
                            self.heap.write_f32(float_value as f32)?;
                        }
                        false => {
                            self.glob_data.set_current_byte_idx(address);
                            self.glob_data.write_f32(float_value as f32)?;
                        }
                    };
                }
                Opcode::F64Store => {
                    let glob_idx = self.code.read_u32()?;
                    let glob_ref = match self.glob_refs.get(glob_idx as usize) {
                        Some(glob_ref) => glob_ref,
                        None => return Err(String::from(format!("Unexpected glob idx '{:X}'", glob_idx)))
                    };

                    // Read u64 representation of float and just write those bytes
                    let value = self.stack.pop()?;
                    self.glob_data.set_current_byte_idx(glob_ref.offset as usize);
                    self.glob_data.write_u64(value)?;
                }
                Opcode::F64StoreC => {
                    let mut address = self.stack.pop()? as usize;
                    let value = self.stack.pop()?;
                    let float_value = f64::from_le_bytes(value.to_le_bytes());

                    match address >= self.glob_data_size {
                        true => {
                            address = address - self.glob_data_size;
                            if address >= self.heap_size as usize {
                                return Err(String::from("Heap overflow"));
                            }
                            self.heap.set_current_byte_idx(address);
                            self.heap.write_f64(float_value)?;
                        }
                        false => {
                            self.glob_data.set_current_byte_idx(address);
                            self.glob_data.write_f64(float_value)?;
                        }
                    };
                }
                Opcode::I64Eqz => {
                    let value = self.stack.pop()?;
                    let offset = self.code.read_u64()?;
                    if value != 0 {
                        let program_counter = self.code.get_current_byte_idx();
                        self.code.set_current_byte_idx(program_counter + offset as usize);
                    }
                }
                Opcode::I64Eq => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    let offset = self.code.read_u64()?;
                    if lhs != rhs {
                        let program_counter = self.code.get_current_byte_idx();
                        self.code.set_current_byte_idx(program_counter + offset as usize);
                    }
                }
                Opcode::I64Ne => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    let offset = self.code.read_u64()?;
                    if lhs == rhs {
                        let program_counter = self.code.get_current_byte_idx();
                        self.code.set_current_byte_idx(program_counter + offset as usize);
                    }
                }
                Opcode::I64LtS => {
                    let lhs = self.stack.pop()? as i64;
                    let rhs = self.stack.pop()? as i64;
                    let offset = self.code.read_u64()?;
                    if lhs >= rhs {
                        let program_counter = self.code.get_current_byte_idx();
                        self.code.set_current_byte_idx(program_counter + offset as usize);
                    }
                }
                Opcode::I64LtU => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    let offset = self.code.read_u64()?;
                    if lhs >= rhs {
                        let program_counter = self.code.get_current_byte_idx();
                        self.code.set_current_byte_idx(program_counter + offset as usize);
                    }
                }
                Opcode::I64GtS => {
                    let lhs = self.stack.pop()? as i64;
                    let rhs = self.stack.pop()? as i64;
                    let offset = self.code.read_u64()?;
                    if lhs <= rhs {
                        let program_counter = self.code.get_current_byte_idx();
                        self.code.set_current_byte_idx(program_counter + offset as usize);
                    }
                }
                Opcode::I64GtU => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    let offset = self.code.read_u64()?;
                    if lhs <= rhs {
                        let program_counter = self.code.get_current_byte_idx();
                        self.code.set_current_byte_idx(program_counter + offset as usize);
                    }
                }
                Opcode::I64LeS => {
                    let lhs = self.stack.pop()? as i64;
                    let rhs = self.stack.pop()? as i64;
                    let offset = self.code.read_u64()?;
                    if lhs > rhs {
                        let program_counter = self.code.get_current_byte_idx();
                        self.code.set_current_byte_idx(program_counter + offset as usize);
                    }
                }
                Opcode::I64LeU => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    let offset = self.code.read_u64()?;
                    if lhs > rhs {
                        let program_counter = self.code.get_current_byte_idx();
                        self.code.set_current_byte_idx(program_counter + offset as usize);
                    }
                }
                Opcode::I64GeS => {
                    let lhs = self.stack.pop()? as i64;
                    let rhs = self.stack.pop()? as i64;
                    let offset = self.code.read_u64()?;
                    if lhs < rhs {
                        let program_counter = self.code.get_current_byte_idx();
                        self.code.set_current_byte_idx(program_counter + offset as usize);
                    }
                }
                Opcode::I64GeU => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    let offset = self.code.read_u64()?;
                    if lhs < rhs {
                        let program_counter = self.code.get_current_byte_idx();
                        self.code.set_current_byte_idx(program_counter + offset as usize);
                    }
                }
                Opcode::F64Eq => {
                    let lhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    let rhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    let offset = self.code.read_u64()?;
                    if lhs != rhs {
                        let program_counter = self.code.get_current_byte_idx();
                        self.code.set_current_byte_idx(program_counter + offset as usize);
                    }
                }
                Opcode::F64Ne => {
                    let lhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    let rhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    let offset = self.code.read_u64()?;
                    if lhs == rhs {
                        let program_counter = self.code.get_current_byte_idx();
                        self.code.set_current_byte_idx(program_counter + offset as usize);
                    }
                }
                Opcode::F64Lt => {
                    let lhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    let rhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    let offset = self.code.read_u64()?;
                    if lhs >= rhs {
                        let program_counter = self.code.get_current_byte_idx();
                        self.code.set_current_byte_idx(program_counter + offset as usize);
                    }
                }
                Opcode::F64Gt => {
                    let lhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    let rhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    let offset = self.code.read_u64()?;
                    if lhs <= rhs {
                        let program_counter = self.code.get_current_byte_idx();
                        self.code.set_current_byte_idx(program_counter + offset as usize);
                    }
                }
                Opcode::F64Le => {
                    let lhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    let rhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    let offset = self.code.read_u64()?;
                    if lhs > rhs {
                        let program_counter = self.code.get_current_byte_idx();
                        self.code.set_current_byte_idx(program_counter + offset as usize);
                    }
                }
                Opcode::F64Ge => {
                    let lhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    let rhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    let offset = self.code.read_u64()?;
                    if lhs < rhs {
                        let program_counter = self.code.get_current_byte_idx();
                        self.code.set_current_byte_idx(program_counter + offset as usize);
                    }
                }
                Opcode::I64Add => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    self.stack.push(lhs + rhs)?;
                }
                Opcode::I64Sub => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    self.stack.push(lhs - rhs)?;
                }
                Opcode::I64Mul => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    self.stack.push(lhs * rhs)?;
                }
                Opcode::I64DivS => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    self.stack.push(((lhs as i64) / (rhs as i64)) as u64)?;
                }
                Opcode::I64DivU => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    self.stack.push(lhs / rhs)?;
                }
                Opcode::I64RemS => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    self.stack.push(((lhs as i64) % (rhs as i64)) as u64)?;
                }
                Opcode::I64RemU => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    self.stack.push(lhs % rhs)?;
                }
                Opcode::I64Pow => {
                    let value = self.stack.pop()?;
                    let pow = self.stack.pop()?;
                    self.stack.push(u64::pow(value, pow as u32))?;
                }
                Opcode::I64Abs => {
                    let value = self.stack.pop()?;
                    self.stack.push((value as i64).abs() as u64)?;
                }
                Opcode::I64Sqrt => {
                    let value = self.stack.pop()?;
                    let float_value = f64::from_le_bytes(value.to_le_bytes());
                    let result = float_value.sqrt();
                    self.stack.push(u64::from_le_bytes(result.to_le_bytes()))?;
                }
                Opcode::I64And => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    self.stack.push(lhs & rhs)?;
                }
                Opcode::I64Or => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    self.stack.push(lhs | rhs)?;
                }
                Opcode::I64Xor => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    self.stack.push(lhs ^ rhs)?;
                }
                Opcode::I64Shl => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    self.stack.push(lhs << rhs)?;
                }
                Opcode::I64ShrS => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    match (lhs as i64) < 0 {
                        true => self.stack.push(0x8000000000000000 | lhs >> rhs)?,
                        false => self.stack.push(lhs >> rhs)?
                    };
                }
                Opcode::I64ShrU => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    self.stack.push(lhs >> rhs)?;
                }
                Opcode::I64Rotl => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    self.stack.push(lhs << rhs | lhs >> (64 - rhs))?;
                }
                Opcode::I64Rotr => {
                    let lhs = self.stack.pop()?;
                    let rhs = self.stack.pop()?;
                    self.stack.push(lhs >> rhs | lhs << (64 - rhs))?;
                }
                Opcode::F64Add => {
                    let lhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    let rhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    self.stack.push(u64::from_le_bytes((lhs + rhs).to_le_bytes()))?;
                }
                Opcode::F64Sub => {
                    let lhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    let rhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    self.stack.push(u64::from_le_bytes((lhs - rhs).to_le_bytes()))?;
                }
                Opcode::F64Mul => {
                    let lhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    let rhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    self.stack.push(u64::from_le_bytes((lhs * rhs).to_le_bytes()))?;
                }
                Opcode::F64Div => {
                    let lhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    let rhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    self.stack.push(u64::from_le_bytes((lhs / rhs).to_le_bytes()))?;
                }
                Opcode::F64Pow => {
                    let lhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    let rhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    self.stack.push(u64::from_le_bytes((lhs % rhs).to_le_bytes()))?;
                }
                Opcode::F64Abs => {
                    let lhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    self.stack.push(u64::from_le_bytes(lhs.abs().to_le_bytes()))?;
                }
                Opcode::F64Ceil => {
                    let lhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    self.stack.push(u64::from_le_bytes(lhs.ceil().to_le_bytes()))?;
                }
                Opcode::F64Floor => {
                    let lhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    self.stack.push(u64::from_le_bytes(lhs.floor().to_le_bytes()))?;
                }
                Opcode::F64Trunc => {
                    let lhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    self.stack.push(u64::from_le_bytes(lhs.trunc().to_le_bytes()))?;
                }
                Opcode::F64Nearest => {
                    return Err(String::from(format!("Unimplemented instruction F64Nearest")));
                }
                Opcode::F64Sqrt => {
                    let lhs = f64::from_le_bytes(self.stack.pop()?.to_le_bytes());
                    self.stack.push(u64::from_le_bytes(lhs.sqrt().to_le_bytes()))?;
                }
                unknown => {
                    return Err(String::from(format!("Unimplemented instruction '{:X}'", unknown as u8)));
                }
            }
        }
    }
}