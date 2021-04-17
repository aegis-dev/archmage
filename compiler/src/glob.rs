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

use crate::out_bin::OutBin;

pub struct Glob {
    name: String,
    value: GlobalValue,
}

pub enum GlobalValue {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    ByteArray(Vec<u8>)
}

impl Glob {
    pub fn new(name: String) -> Glob {
        Glob { name, value: GlobalValue::U8(0) }
    }

    pub fn new_with_value(name: String, value: GlobalValue) -> Glob {
        Glob { name, value }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn encode(&self, bin: &mut OutBin) -> Result<(), String> {
        let mut glob_data: Vec<u8> = vec![];

        match &self.value {
            GlobalValue::U8(value) => {
                let bytes = value.to_le_bytes();
                glob_data.extend_from_slice(&bytes);
            }
            GlobalValue::U16(value) => {
                let bytes = value.to_le_bytes();
                glob_data.extend_from_slice(&bytes);
            }
            GlobalValue::U32(value) => {
                let bytes = value.to_le_bytes();
                glob_data.extend_from_slice(&bytes);
            }
            GlobalValue::U64(value) => {
                let bytes = value.to_le_bytes();
                glob_data.extend_from_slice(&bytes);
            }
            GlobalValue::F32(value) => {
                let bytes = value.to_le_bytes();
                glob_data.extend_from_slice(&bytes);
            }
            GlobalValue::F64(value) => {
                let bytes = value.to_le_bytes();
                glob_data.extend_from_slice(&bytes);
            }
            GlobalValue::ByteArray(value) => {
                glob_data.extend_from_slice(&value);
            }
        }

        let glob_offset = bin.add_glob_data(&glob_data);
        match bin.get_glob_mut_ref(self.get_name()) {
            Some(glob_ref) => {
                glob_ref.size = glob_data.len() as u32;
                glob_ref.offset = glob_offset;
            }
            None => {
                return Err(String::from(format!("No func in funcs table with name '{0}'", self.get_name())));
            }
        }

        Ok(())
    }
}