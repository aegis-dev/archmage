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

pub struct ByteVecReader {
    byte_vec: Vec<u8>,
    current_byte_idx: usize,
}

impl ByteVecReader {
    pub fn new(byte_vec: Vec<u8>) -> ByteVecReader {
        ByteVecReader { byte_vec, current_byte_idx: 0 }
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
}