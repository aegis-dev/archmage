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

#[derive(PartialEq)]
pub enum Literal {
    None(),
    Offset(u32),
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
    pub fn decode() -> Instruction {
        // TODO
        Instruction { opcode: Opcode::Nop, literal: Literal::None() }
    }

    pub fn encode(&self, code: &mut Vec<u8>, bin: &OutBin) -> Result<(), String> {
        match &self.literal {
            Literal::None() => {}
            Literal::Offset(value) => {
                code.push(self.opcode as u8);
                let bytes = value.to_le_bytes();
                code.extend_from_slice(&bytes);
            }
            Literal::Const(value) => {
                code.push(self.opcode as u8);
                let bytes = value.to_le_bytes();
                code.extend_from_slice(&bytes);
            }
            Literal::FloatConst(value) => {
                code.push(self.opcode as u8);

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
            Literal::Jump(_dest_label) => {
                code.push(self.opcode as u8);
                // label offset will be updated after all instructions are encoded
                let bytes: [u8; 4] = [0, 0, 0, 0];
                code.extend_from_slice(&bytes);
            }
            Literal::Label(_) => {
                // skip this pseudo instruction - nothing to encode since this is jump destination
            }
        }

        Ok(())
    }

    pub fn get_literal(&self) -> &Literal {
        &self.literal
    }

    pub fn get_mut_literal(&mut self) -> &mut Literal {
        &mut self.literal
    }
}