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

use crate::instruction::{Instruction, Literal};
use crate::out_bin::OutBin;

pub struct Func {
    name: String,
    result_count: u8,
    code: Vec<Instruction>
}

impl Func {
    pub fn new(name: String, result_count: u8) -> Func {
        Func { name, result_count, code: vec![] }
    }

    pub fn new_with_code(name: String, result_count: u8, code: Vec<Instruction>) -> Func {
        Func { name, result_count, code }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_code(&mut self, code: Vec<Instruction>) {
        self.code = code;
    }

    pub fn get_code(&self) -> &Vec<Instruction> {
        &self.code
    }

    pub fn get_mut_code(&mut self) -> &mut Vec<Instruction> {
        &mut self.code
    }

    pub fn encode(&self, bin: &mut OutBin) -> Result<(), String> {
        let mut code: Vec<u8> = vec![];

        let mut label_dests: HashMap<String, u64> = HashMap::new();
        let mut jumps_to_update: HashMap<u64, String>  = HashMap::new();

        for instruction in self.code.iter() {
            instruction.encode(&mut code, &bin, &mut label_dests, &mut jumps_to_update)?;
        }

        for (offset, label_dest) in jumps_to_update.iter() {
            match label_dests.get(label_dest) {
                Some(dest_offset) => {
                    let bytes = dest_offset.to_le_bytes();
                    let offset_as_idx = *offset as usize;
                    code[offset_as_idx + 0] = bytes[0];
                    code[offset_as_idx + 1] = bytes[1];
                    code[offset_as_idx + 2] = bytes[2];
                    code[offset_as_idx + 3] = bytes[3];
                    code[offset_as_idx + 4] = bytes[4];
                    code[offset_as_idx + 5] = bytes[5];
                    code[offset_as_idx + 6] = bytes[6];
                    code[offset_as_idx + 7] = bytes[7];
                }
                None => {
                    return Err(String::from(format!("Unknown label '{0}' in method {1}", label_dest, self.get_name())));
                }
            }
        }

        let func_code_offset = bin.add_func_code(&code);
        match bin.get_func_mut_ref(self.get_name()) {
            Some(func_ref) => {
                func_ref.size = code.len() as u32;
                func_ref.offset = func_code_offset;
                func_ref.result_count = self.result_count;
            }
            None => {
                return Err(String::from(format!("No func in funcs table with name '{0}'", self.get_name())));
            }
        }

        Ok(())
    }
}