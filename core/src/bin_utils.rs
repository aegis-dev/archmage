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

use std::mem;
use std::io::{BufReader, Read};
use std::fs::File;

pub fn read_u8(reader: &mut BufReader<File>) -> Result<u8, String> {
    let mut byte: [u8; mem::size_of::<u8>()] = [0; mem::size_of::<u8>()];
    match reader.read_exact(&mut byte) {
        Ok(_) => Ok(byte[0]),
        Err(_) => Err(String::from("Failed to read bytes"))
    }
}

pub fn read_u16(reader: &mut BufReader<File>) -> Result<u16, String> {
    let mut bytes: [u8; mem::size_of::<u16>()] = [0; mem::size_of::<u16>()];
    match reader.read_exact(&mut bytes) {
        Ok(_) => Ok(u16::from_le_bytes(bytes)),
        Err(_) => Err(String::from("Failed to read bytes"))
    }
}

pub fn read_u32(reader: &mut BufReader<File>) -> Result<u32, String> {
    let mut bytes: [u8; mem::size_of::<u32>()] = [0; mem::size_of::<u32>()];
    match reader.read_exact(&mut bytes) {
        Ok(_) => Ok(u32::from_le_bytes(bytes)),
        Err(_) => Err(String::from("Failed to read bytes"))
    }
}

pub fn read_u64(reader: &mut BufReader<File>) -> Result<u64, String> {
    let mut bytes: [u8; mem::size_of::<u64>()] = [0; mem::size_of::<u64>()];
    match reader.read_exact(&mut bytes) {
        Ok(_) => Ok(u64::from_le_bytes(bytes)),
        Err(_) => Err(String::from("Failed to read bytes"))
    }
}

pub fn read_f32(reader: &mut BufReader<File>) -> Result<f32, String> {
    let mut bytes: [u8; mem::size_of::<f32>()] = [0; mem::size_of::<f32>()];
    match reader.read_exact(&mut bytes) {
        Ok(_) => Ok(f32::from_le_bytes(bytes)),
        Err(_) => Err(String::from("Failed to read bytes"))
    }
}

pub fn read_f64(reader: &mut BufReader<File>) -> Result<f64, String> {
    let mut bytes: [u8; mem::size_of::<f64>()] = [0; mem::size_of::<f64>()];
    match reader.read_exact(&mut bytes) {
        Ok(_) => Ok(f64::from_le_bytes(bytes)),
        Err(_) => Err(String::from("Failed to read bytes"))
    }
}

pub fn read_bytes(reader: &mut BufReader<File>, bytes: &mut [u8]) -> Result<(), String> {
    match reader.read_exact(bytes) {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("Failed to read bytes"))
    }
}

pub fn str_from_table(str_table: &Vec<u8>, offset: u32) -> Result<String, String> {
    let mut str_bytes = vec![];
    let mut idx = offset as usize;
    'str_parse: loop {
        match str_table.get(idx) {
            Some(byte) => {
                if *byte == '\0' as u8 {
                    break 'str_parse;
                }
                str_bytes.push(*byte);
            }
            None => {
                return Err(String::from(format!("Failed to parse string from string table at offset of '{}'", offset)))
            }
        };
        idx += 1;
    }
    match String::from_utf8(str_bytes) {
        Ok(string) => Ok(string),
        Err(_) =>  Err(String::from(format!("Failed to parse string from string table at offset of '{}'", offset)))
    }
}
