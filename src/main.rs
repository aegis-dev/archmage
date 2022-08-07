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

mod memory;
mod sys_call;
mod palette;
mod video_mode;
mod input;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::convert::TryFrom;

use shard_vm::vm::{
    VM, ExecutionStatus
};

use flask::flask_context::FlaskContext;
use flask::frame_buffer::FrameBuffer;

use crate::memory::{MachineMemory, VIDEO_BUFFER_WIDTH, VIDEO_BUFFER_HEIGHT, VIDEO_MODE, VIDEO_BUFFER_START, VIDEO_BUFFER_SIZE, STACK_START, STACK_SIZE, CURSOR_POSITION_Y, CURSOR_POSITION_X};
use crate::sys_call::SysCall;
use crate::video_mode::VideoMode;

fn main() {
    match run_machine() {
        Ok(_) => { }
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn print_help() {
    println!("archmage [kernel.bin]");
    println!("archmage --help");
    println!("archmage --sys-info");
}

fn print_sys_info() {
    println!("--------------------- CPU ----------------------");
    println!("8 bit execution mode");
    println!("16 bit addressing");
    println!("Powered by shard_lang - toy assembly language and VM");
    println!();
    println!("----------------- Screen size ------------------");
    println!("VIDEO_BUFFER_WIDTH    {0} | 0x{1:x}", VIDEO_BUFFER_WIDTH, VIDEO_BUFFER_WIDTH);
    println!("VIDEO_BUFFER_HEIGHT   {0} | 0x{1:x}", VIDEO_BUFFER_HEIGHT, VIDEO_BUFFER_HEIGHT);
    println!("VIDEO_BUFFER_SIZE     {0} | 0x{1:x}", VIDEO_BUFFER_SIZE, VIDEO_BUFFER_SIZE);
    println!();
    println!("---------------- Memory Layout -----------------");
    println!("STACK_START           0x{0:x} - 0x{1:x}", STACK_START, STACK_START + STACK_SIZE);
    println!("VIDEO_BUFFER_START    0x{0:x} - 0x{1:x}", VIDEO_BUFFER_START, VIDEO_BUFFER_START + VIDEO_BUFFER_SIZE);
    println!("VIDEO_MODE            0x{0:x}", VIDEO_MODE);
    println!("CURSOR_POSITION_Y     0x{0:x}", CURSOR_POSITION_Y);
    println!("CURSOR_POSITION_X     0x{0:x}", CURSOR_POSITION_X);
    println!();
    println!("------------------------------------------------");
}

pub fn run_machine() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    if args[1] == "--help" {
        print_help();
        return Ok(());
    } else if args[1] == "--sys-info" {
        print_sys_info();
        return Ok(());
    } else if args[1].starts_with("-") {
        print_help();
        return Ok(());
    }

    let kernel_file = args[1].clone();
    let kernel_bytes = match read_file_bytes(&kernel_file) {
        Ok(kernel_bytes) => kernel_bytes,
        Err(err) => {
            return Err(String::from(format!("{}", err)));
        }
    };

    let memory = match MachineMemory::new(kernel_bytes) {
        Ok(memory) => Box::new(memory),
        Err(err) =>  {
            return Err(String::from(format!("Failed to load kernel: {}", err)));
        }
    };

    let mut flask_context = match FlaskContext::new(
        VIDEO_BUFFER_WIDTH as u32,
        VIDEO_BUFFER_HEIGHT as u32,
        "archmage",
        palette::archmage_palette()
    ) {
        Ok(context) => context,
        Err(err) => {
            return Err(String::from(format!("Failed to create window context: {}", err)));
        }
    };

    let mut custom_frame_buffer = FrameBuffer::new(VIDEO_BUFFER_WIDTH as u32, VIDEO_BUFFER_HEIGHT as u32);

    let mut vm = VM::new(memory);

    loop {
        let status = match vm.execute_instruction() {
            Ok(status) => status,
            Err(err) => {
                return Err(String::from(format!("ERROR: {}", err)));
            }
        };

        let sys_call = match status {
            ExecutionStatus::Ok => continue,
            ExecutionStatus::Interrupt => return Ok(()),
            ExecutionStatus::SysCall => {
                match SysCall::try_from(vm.stack_pop().unwrap()) {
                    Ok(sys_call) => sys_call,
                    Err(_) => {
                        return Err(String::from("ERROR: unknown syscall"));
                    }
                }
            }
        };

        match sys_call {
            SysCall::None => { }
            SysCall::RenderVRAM => {
                let video_mode = match VideoMode::try_from(vm.peek_memory(VIDEO_MODE).unwrap()) {
                    Ok(video_mode) => video_mode,
                    Err(_) => VideoMode::Pixel
                };

                let vram = vm.dump_memory_range(VIDEO_BUFFER_START, VIDEO_BUFFER_START + VIDEO_BUFFER_SIZE);

                match video_mode {
                    VideoMode::Pixel => {
                        let mut column = 0;
                        let mut line = 0;
                        for duo_pixel in vram {
                            custom_frame_buffer.set_pixel(line, column, duo_pixel & 0x0f);
                            custom_frame_buffer.set_pixel(line, column + 1, (duo_pixel & 0xf0) >> 4);
                            column += 2;
                            if column >= VIDEO_BUFFER_WIDTH as u32 {
                                column = 0;
                                line += 1;
                            }
                        }
                    }
                    VideoMode::Text => {
                        //TODO:
                    }
                }

                flask_context.render_buffer_and_swap(&custom_frame_buffer)?;
            }
            SysCall::PollInputEvents => {
                // Polling input events. If there are any key input press events pushed to the stack,
                // we also push 0x01 to the stack, otherwise, pushing 0x00.
                match flask_context.poll_sdl_input_event() {
                    Some(event) => {
                        match input::process_sdl_event(&mut vm, &event)? {
                            Some(_) => {
                                vm.stack_push(0x01)?;
                            }
                            None => {
                                vm.stack_push(0x00)?;
                            }
                        }
                    }
                    None => {
                        vm.stack_push(0x00)?;
                    }
                }

            }
            // SysCall::UpdateCursorState => {}
            // SysCall::GetMouseButtonState => {}
        }
    }
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
