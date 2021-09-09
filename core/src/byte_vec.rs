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

pub struct ByteVec {
    byte_vec: Vec<u8>,
    current_byte_idx: usize,
}

impl ByteVec {
    pub fn new(byte_vec: Vec<u8>) -> ByteVec {
        ByteVec { byte_vec, current_byte_idx: 0 }
    }

    pub fn len(&self) -> usize {
        self.byte_vec.len()
    }

    pub fn resize(&mut self, len: usize) {
        self.byte_vec.resize(len, 0);
    }

    pub fn set_current_byte_idx(&mut self, current_byte_idx: usize) {
        self.current_byte_idx = current_byte_idx;
    }

    pub fn get_current_byte_idx(&self) -> usize {
        self.current_byte_idx
    }

    pub fn read_u8(&mut self) -> Result<u8, String> {
        match self.byte_vec.get(self.current_byte_idx) {
            Some(byte) => {
                self.current_byte_idx += 1;
                Ok(*byte)
            }
            None =>  return Err(String::from("Index out of bounds"))
        }
    }

    pub fn read_u16(&mut self) -> Result<u16, String> {
        let bytes = [self.read_u8()?, self.read_u8()?];
        Ok(u16::from_le_bytes(bytes))
    }

    pub fn read_u32(&mut self) -> Result<u32, String> {
        let bytes = [self.read_u8()?, self.read_u8()?, self.read_u8()?, self.read_u8()?];
        Ok(u32::from_le_bytes(bytes))
    }

    pub fn read_u64(&mut self) -> Result<u64, String> {
        let bytes = [
            self.read_u8()?, self.read_u8()?, self.read_u8()?, self.read_u8()?,
            self.read_u8()?, self.read_u8()?, self.read_u8()?, self.read_u8()?
        ];
        Ok(u64::from_le_bytes(bytes))
    }

    pub fn read_f32(&mut self) -> Result<f32, String> {
        let bytes = [self.read_u8()?, self.read_u8()?, self.read_u8()?, self.read_u8()?];
        Ok(f32::from_le_bytes(bytes))
    }

    pub fn read_f64(&mut self) -> Result<f64, String> {
        let bytes = [
            self.read_u8()?, self.read_u8()?, self.read_u8()?, self.read_u8()?,
            self.read_u8()?, self.read_u8()?, self.read_u8()?, self.read_u8()?
        ];
        Ok(f64::from_le_bytes(bytes))
    }

    pub fn write_u8(&mut self, value: u8) -> Result<(), String> {
        if self.current_byte_idx >= self.len() {
            return Err(String::from("Index out of bounds"));
        }
        self.byte_vec[self.current_byte_idx] = value;
        self.current_byte_idx += 1;
        Ok(())
    }

    pub fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), String> {
        for b in bytes {
            self.write_u8(*b)?;
        }
        Ok(())
    }

    pub fn write_u16(&mut self, value: u16) -> Result<(), String> {
        let bytes = value.to_le_bytes();
        self.write_bytes(&bytes)
    }

    pub fn write_u32(&mut self, value: u32) -> Result<(), String> {
        let bytes = value.to_le_bytes();
        self.write_bytes(&bytes)
    }

    pub fn write_u64(&mut self, value: u64) -> Result<(), String> {
        let bytes = value.to_le_bytes();
        self.write_bytes(&bytes)
    }

    pub fn write_f32(&mut self, value: f32) -> Result<(), String> {
        let bytes = value.to_le_bytes();
        self.write_bytes(&bytes)
    }

    pub fn write_f64(&mut self, value: f64) -> Result<(), String> {
        let bytes = value.to_le_bytes();
        self.write_bytes(&bytes)
    }
}