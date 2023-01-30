//
// Copyright © 2020-2023  Egidijus Lileika
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

use shard_vm::{memory::Memory, vm::VM_MAX_IMAGE_SIZE};

pub const IMAGE_SIZE: usize = VM_MAX_IMAGE_SIZE;

pub const VIDEO_BUFFER_WIDTH: u16 = 256;
pub const VIDEO_BUFFER_HEIGHT: u16 = 144;

// Memory layout
pub const STACK_START: u16 = 0xff00;
pub const STACK_SIZE: u16 = u8::MAX as u16 + 1;

pub const CALL_STACK_START: u16 = STACK_START - CALL_STACK_SIZE;
pub const CALL_STACK_SIZE: u16 = u8::MAX as u16 + 1;

pub const VIDEO_BUFFER_START: u16 = CALL_STACK_START - VIDEO_BUFFER_SIZE;
pub const VIDEO_BUFFER_SIZE: u16 = VIDEO_BUFFER_WIDTH * VIDEO_BUFFER_HEIGHT / 2;

pub const VIDEO_MODE: u16 = VIDEO_BUFFER_START - 1;
pub const CURSOR_POSITION_Y: u16 = VIDEO_MODE - 1;
pub const CURSOR_POSITION_X: u16 = CURSOR_POSITION_Y - 1;

// Last element of memory layout is the start of reserved memory.
// This memory can't be occupied by kernel code.
pub const RESERVED_MEMORY_START: usize = CURSOR_POSITION_X as usize;

pub struct MachineMemory {
    memory: Vec<u8>,
    ram_start_address: u16,
}

impl MachineMemory {
    pub fn new(kernel_code: Vec<u8>) -> Result<MachineMemory, String> {
        let kernel_size = kernel_code.len();
        if kernel_size >= RESERVED_MEMORY_START {
            return Err(String::from(format!("Kernel code exceeds {} size limit", RESERVED_MEMORY_START)));
        }

        let mut memory = kernel_code;
        let ram_start_address = memory.len() as u16;

        let mut ram = vec![0 as u8; IMAGE_SIZE - memory.len()];
        memory.append(&mut ram);
        assert_eq!(memory.len(), IMAGE_SIZE);

        Ok(MachineMemory {
            memory,
            ram_start_address,
        })
    }
}

impl Memory for MachineMemory {
    fn write_u8(&mut self, address: u16, value: u8) -> Result<(), String> {
        if address < self.ram_start_address {
            return Err(String::from("ABORTING: attempting to write into read-only memory"))
        }

        self.memory[address as usize] = value;
        Ok(())
    }

    fn read_u8(&self, address: u16) -> Result<u8, String> {
        Ok(self.memory[address as usize])
    }

    fn stack_start_address(&self) -> u16 {
        STACK_START
    }

    fn call_stack_start_address(&self) -> u16 {
        CALL_STACK_START
    }

    fn ram_start_address(&self) -> u16 {
        self.ram_start_address
    }

    fn dump_memory(&self) -> Vec<u8> {
        self.memory.clone()
    }

    fn dump_memory_range(&self, start: u16, end: u16) -> Vec<u8> {
        self.memory[start as usize..=end as usize].to_vec()
    }
}