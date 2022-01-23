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

mod memory;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use shard_vm::vm::VM;
use crate::memory::MachineMemory;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("archmage [kernel.bin]");
        return;
    }

    let kernel_file = args[1].clone();
    let mut kernel_bytes = match read_file_bytes(&kernel_file) {
        Ok(kernel_bytes) => kernel_bytes,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let vm = VM::new(Box::new(MachineMemory::new(kernel_bytes)));
}

fn read_file_bytes(file_path: &String) -> Result<Vec<u8>, String> {
    if !Path::new(file_path).exists() {
        return Err(String::from(format!("'{}' doesn't exist", file_path)));
    }

    let mut reader = File::open(file_path).expect("Failed to open file");
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes).expect("Failed to read file");

    Ok(bytes)
}
