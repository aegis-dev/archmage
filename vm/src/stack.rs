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

pub struct Stack {
    stack_ptr: usize,
    stack_size: usize,
    stack: Vec<u64>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack { stack_ptr: 0, stack_size: u16::MAX as usize, stack: vec![0; u16::MAX as usize] }
    }

    pub fn push(&mut self, value: u64) -> Result<(), String> {
        if self.stack_ptr >= self.stack_size {
            return Err(String::from("Stack overflow"));
        }

        self.stack[self.stack_ptr] = value;
        self.stack_ptr += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Result<u64, String> {
        if self.stack_ptr <= 0 {
            return Err(String::from("Stack empty"));
        }

        self.stack_ptr -= 1;
        Ok(self.stack[self.stack_ptr])
    }

    pub fn set(&mut self, stack_offset: u64, value: u64) -> Result<(), String> {
        let stack_idx = self.stack_ptr - stack_offset as usize;
        if stack_idx <= 0 {
            return Err(String::from("Stack index out of bounds"));
        }

        self.stack[stack_idx] = value;

        Ok(())
    }

    pub fn get(&mut self, stack_offset: u64) -> Result<u64, String> {
        let stack_idx = self.stack_ptr - stack_offset as usize;
        if stack_idx <= 0 {
            return Err(String::from("Stack index out of bounds"));
        }

        Ok(self.stack[stack_idx])
    }
}