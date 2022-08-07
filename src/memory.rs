//
// Copyright © 2020-2022  Egidijus Lileika
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

use shard_vm::memory::Memory;

pub const VIDEO_BUFFER_WIDTH: u8 = 254;
pub const VIDEO_BUFFER_HEIGHT: u8 = 142;

pub const ADDRESSABLE_MEMORY_SIZE: usize = u16::MAX as usize + 1;

// Memory layout
pub const STACK_START: u16 = 0xff00;
pub const STACK_SIZE: u16 = 0xff;

pub const VIDEO_BUFFER_START: u16 = STACK_START - VIDEO_BUFFER_SIZE - 1;
pub const VIDEO_BUFFER_SIZE: u16 = VIDEO_BUFFER_WIDTH as u16 * VIDEO_BUFFER_HEIGHT as u16 / 2;

pub const VIDEO_MODE: u16 = VIDEO_BUFFER_START - 1;
pub const CURSOR_POSITION_Y: u16 = VIDEO_MODE - 1;
pub const CURSOR_POSITION_X: u16 = CURSOR_POSITION_Y - 1;

pub struct MachineMemory {
    memory: Vec<u8>,
    writable_memory_start: u16,
}

impl MachineMemory {
    pub fn new(kernel_code: Vec<u8>) -> Result<MachineMemory, String> {
        let kernel_size = kernel_code.len();
        if kernel_size >= ADDRESSABLE_MEMORY_SIZE {
            return Err(String::from(format!("Kernel code exceeds {} size limit", ADDRESSABLE_MEMORY_SIZE)));
        }

        let mut memory = kernel_code;
        let mut padding = vec![0 as u8; ADDRESSABLE_MEMORY_SIZE - memory.len()];
        memory.append(&mut padding);
        assert_eq!(memory.len(), ADDRESSABLE_MEMORY_SIZE);

        Ok(MachineMemory {
            memory,
            writable_memory_start: kernel_size as u16,
        })
    }
}

impl Memory for MachineMemory {
    fn write_u8(&mut self, address: u16, value: u8) -> Result<(), String> {
        if address < self.writable_memory_start {
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

    fn dump_memory(&self) -> Vec<u8> {
        self.memory.clone()
    }

    fn dump_memory_range(&self, start: u16, end: u16) -> Vec<u8> {
        self.memory[start as usize..=end as usize].to_vec()
    }
}