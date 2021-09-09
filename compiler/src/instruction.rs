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

use core::opcodes::Opcode;

use crate::out_bin::OutBin;
use core::byte_vec::ByteVec;
use std::collections::HashMap;
use std::convert::TryFrom;

#[derive(PartialEq)]
pub enum Literal {
    None(),
    Offset(u64),
    Const(u64),
    FloatConst(f64),
    Func(String),
    Glob(String),
    Jump(String),
    Label(String),
}

pub struct Instruction {
    opcode: Opcode,
    literal: Literal,
}

impl Instruction {
    pub fn get_opcode(&self) -> Opcode {
        self.opcode
    }

    pub fn get_literal(&self) -> &Literal {
        &self.literal
    }

    pub fn get_mut_literal(&mut self) -> &mut Literal {
        &mut self.literal
    }

    pub fn decode(
        reader: &mut ByteVec,
        func_name_map: &HashMap<u32, String>,
        glob_name_map: &HashMap<u32, String>
    ) -> Result<(Instruction, usize), String> {

        let offset_before_read = reader.get_current_byte_idx();

        let opcode = match Opcode::try_from(reader.read_u8()?) {
            Ok(opcode) => opcode,
            Err(_) => return Err(String::from(format!("Failed to decode instruction")))
        };

        let literal: Literal = match &opcode {
            Opcode::Nop => Literal::None(),
            Opcode::Return => Literal::None(),
            Opcode::Call => {
                let func_idx = reader.read_u32()?;
                let func_name = func_name_map.get(&func_idx).ok_or_else(||"Function index is out of bounds")?;
                Literal::Func(func_name.clone())
            }
            Opcode::Jump => {
                let jump_offset = reader.read_u64()?;
                // TODO think about reconstructing labels
                Literal::Offset(jump_offset)
            }
            Opcode::JumpC => Literal::None(),
            Opcode::Pop => Literal::None(),
            Opcode::StackGet => {
                let stack_offset = reader.read_u64()?;
                Literal::Offset(stack_offset)
            }
            Opcode::StackSet => {
                let stack_offset = reader.read_u64()?;
                Literal::Offset(stack_offset)
            }
            Opcode::I64Const => {
                let constant = reader.read_u64()?;
                Literal::Const(constant)
            }
            Opcode::F64Const => {
                let constant = reader.read_f64()?;
                Literal::FloatConst(constant)
            }
            Opcode::I8Load => {
                let glob_idx = reader.read_u32()?;
                let glob_name = glob_name_map.get(&glob_idx).ok_or_else(||"Global index is out of bounds")?;
                Literal::Glob(glob_name.clone())
            }
            Opcode::I8LoadC => Literal::None(),
            Opcode::I16Load => {
                let glob_idx = reader.read_u32()?;
                let glob_name = glob_name_map.get(&glob_idx).ok_or_else(||"Global index is out of bounds")?;
                Literal::Glob(glob_name.clone())
            }
            Opcode::I16LoadC => Literal::None(),
            Opcode::I32Load => {
                let glob_idx = reader.read_u32()?;
                let glob_name = glob_name_map.get(&glob_idx).ok_or_else(||"Global index is out of bounds")?;
                Literal::Glob(glob_name.clone())
            }
            Opcode::I32LoadC => Literal::None(),
            Opcode::I64Load => {
                let glob_idx = reader.read_u32()?;
                let glob_name = glob_name_map.get(&glob_idx).ok_or_else(||"Global index is out of bounds")?;
                Literal::Glob(glob_name.clone())
            }
            Opcode::I64LoadC => Literal::None(),
            Opcode::F32Load => {
                let glob_idx = reader.read_u32()?;
                let glob_name = glob_name_map.get(&glob_idx).ok_or_else(||"Global index is out of bounds")?;
                Literal::Glob(glob_name.clone())
            }
            Opcode::F32LoadC => Literal::None(),
            Opcode::F64Load => {
                let glob_idx = reader.read_u32()?;
                let glob_name = glob_name_map.get(&glob_idx).ok_or_else(||"Global index is out of bounds")?;
                Literal::Glob(glob_name.clone())
            }
            Opcode::F64LoadC => Literal::None(),
            Opcode::I8Store => {
                let glob_idx = reader.read_u32()?;
                let glob_name = glob_name_map.get(&glob_idx).ok_or_else(||"Global index is out of bounds")?;
                Literal::Glob(glob_name.clone())
            }
            Opcode::I8StoreC => Literal::None(),
            Opcode::I16Store => {
                let glob_idx = reader.read_u32()?;
                let glob_name = glob_name_map.get(&glob_idx).ok_or_else(||"Global index is out of bounds")?;
                Literal::Glob(glob_name.clone())
            }
            Opcode::I16StoreC => Literal::None(),
            Opcode::I32Store => {
                let glob_idx = reader.read_u32()?;
                let glob_name = glob_name_map.get(&glob_idx).ok_or_else(||"Global index is out of bounds")?;
                Literal::Glob(glob_name.clone())
            }
            Opcode::I32StoreC => Literal::None(),
            Opcode::I64Store => {
                let glob_idx = reader.read_u32()?;
                let glob_name = glob_name_map.get(&glob_idx).ok_or_else(||"Global index is out of bounds")?;
                Literal::Glob(glob_name.clone())
            }
            Opcode::I64StoreC => Literal::None(),
            Opcode::F32Store => {
                let glob_idx = reader.read_u32()?;
                let glob_name = glob_name_map.get(&glob_idx).ok_or_else(||"Global index is out of bounds")?;
                Literal::Glob(glob_name.clone())
            }
            Opcode::F32StoreC => Literal::None(),
            Opcode::F64Store => {
                let glob_idx = reader.read_u32()?;
                let glob_name = glob_name_map.get(&glob_idx).ok_or_else(||"Global index is out of bounds")?;
                Literal::Glob(glob_name.clone())
            }
            Opcode::F64StoreC => Literal::None(),
            Opcode::I64Eqz => {
                let jump_offset = reader.read_u64()?;
                Literal::Offset(jump_offset)
            },
            Opcode::I64Eq => {
                let jump_offset = reader.read_u64()?;
                Literal::Offset(jump_offset)
            },
            Opcode::I64Ne => {
                let jump_offset = reader.read_u64()?;
                Literal::Offset(jump_offset)
            },
            Opcode::I64LtS => {
                let jump_offset = reader.read_u64()?;
                Literal::Offset(jump_offset)
            },
            Opcode::I64LtU => {
                let jump_offset = reader.read_u64()?;
                Literal::Offset(jump_offset)
            },
            Opcode::I64GtS => {
                let jump_offset = reader.read_u64()?;
                Literal::Offset(jump_offset)
            },
            Opcode::I64GtU => {
                let jump_offset = reader.read_u64()?;
                Literal::Offset(jump_offset)
            },
            Opcode::I64LeS => {
                let jump_offset = reader.read_u64()?;
                Literal::Offset(jump_offset)
            },
            Opcode::I64LeU => {
                let jump_offset = reader.read_u64()?;
                Literal::Offset(jump_offset)
            },
            Opcode::I64GeS => {
                let jump_offset = reader.read_u64()?;
                Literal::Offset(jump_offset)
            },
            Opcode::I64GeU => {
                let jump_offset = reader.read_u64()?;
                Literal::Offset(jump_offset)
            },
            Opcode::F64Eq => {
                let jump_offset = reader.read_u64()?;
                Literal::Offset(jump_offset)
            },
            Opcode::F64Ne => {
                let jump_offset = reader.read_u64()?;
                Literal::Offset(jump_offset)
            },
            Opcode::F64Lt => {
                let jump_offset = reader.read_u64()?;
                Literal::Offset(jump_offset)
            },
            Opcode::F64Gt =>{
                let jump_offset = reader.read_u64()?;
                Literal::Offset(jump_offset)
            },
            Opcode::F64Le =>{
                let jump_offset = reader.read_u64()?;
                Literal::Offset(jump_offset)
            },
            Opcode::F64Ge => {
                let jump_offset = reader.read_u64()?;
                Literal::Offset(jump_offset)
            },
            Opcode::I64Add => Literal::None(),
            Opcode::I64Sub => Literal::None(),
            Opcode::I64Mul => Literal::None(),
            Opcode::I64DivS => Literal::None(),
            Opcode::I64DivU => Literal::None(),
            Opcode::I64RemS => Literal::None(),
            Opcode::I64RemU => Literal::None(),
            Opcode::I64Pow => Literal::None(),
            Opcode::I64Abs => Literal::None(),
            Opcode::I64Sqrt => Literal::None(),
            Opcode::I64And => Literal::None(),
            Opcode::I64Or => Literal::None(),
            Opcode::I64Xor => Literal::None(),
            Opcode::I64Shl => Literal::None(),
            Opcode::I64ShrS => Literal::None(),
            Opcode::I64ShrU => Literal::None(),
            Opcode::I64Rotl => Literal::None(),
            Opcode::I64Rotr => Literal::None(),
            Opcode::F64Add => Literal::None(),
            Opcode::F64Sub => Literal::None(),
            Opcode::F64Mul => Literal::None(),
            Opcode::F64Div => Literal::None(),
            Opcode::F64Pow => Literal::None(),
            Opcode::F64Abs => Literal::None(),
            Opcode::F64Ceil => Literal::None(),
            Opcode::F64Floor => Literal::None(),
            Opcode::F64Trunc => Literal::None(),
            Opcode::F64Nearest => Literal::None(),
            Opcode::F64Sqrt => Literal::None(),
            _ => {
                return Err(String::from(format!("Opcode '{}' is a pseudo opcode and shouldn't have been in a binary", opcode.to_string())))
            }
        };

        let instruction_size = reader.get_current_byte_idx() - offset_before_read;
        Ok((Instruction { opcode, literal }, instruction_size))
    }

    pub fn encode(
        &self,
        code: &mut Vec<u8>,
        bin: &OutBin,
        label_dests: &mut HashMap<String, u64>,
        jumps_to_update: &mut HashMap<u64, String>
    ) -> Result<(), String> {
        match &self.literal {
            Literal::Label(label) => {
                // Pseudo instruction - nothing to encode since this is jump destination.
                // Putting destination into the map.
                label_dests.insert(label.clone(), code.len() as u64);
                return Ok(());
            }
            _ => {}
        }

        code.push(self.opcode as u8);

        if Opcode::is_opcode_instruction(self.opcode) {
            // No literal - return
            return Ok(());
        }

        match &self.literal {
            Literal::None() => { }
            Literal::Label(_) => {
                // Pseudo instruction - nothing to encode since this is jump destination.
            }
            Literal::Offset(value) | Literal::Const(value) => {
                let bytes = value.to_le_bytes();
                code.extend_from_slice(&bytes);
            }
            Literal::FloatConst(value) => {
                let bytes = value.to_le_bytes();
                code.extend_from_slice(&bytes);
            }
            Literal::Func(value) => {
                match bin.get_func_idx(value) {
                    Some(func_idx) => {
                        let bytes = func_idx.to_le_bytes();
                        code.extend_from_slice(&bytes);
                    },
                    None => return Err(String::from(format!("No func named '{}'", value)))
                };

            }
            Literal::Glob(value) => {
                match bin.get_glob_idx(value) {
                    Some(glob_idx) =>  {
                        let bytes = glob_idx.to_le_bytes();
                        code.extend_from_slice(&bytes);
                    },
                    None => return Err(String::from(format!("No glob named '{}'", value)))
                };
            }
            Literal::Jump(dest_label) => {
                // Saving position to update later
                jumps_to_update.insert(code.len() as u64, dest_label.clone());

                // label offset will be updated after all instructions are encoded
                let bytes: [u8; 8] = [0; 8];
                code.extend_from_slice(&bytes);
            }
        }

        Ok(())
    }
}