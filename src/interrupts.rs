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

use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use flask::{flask_context::FlaskContext, frame_buffer::FrameBuffer};
use shard_vm::vm::{VM, InterruptType};

use crate::{video_mode::VideoMode, memory::{VIDEO_MODE, VIDEO_BUFFER_START, VIDEO_BUFFER_SIZE, VIDEO_BUFFER_WIDTH}};


pub fn interrupt_handler(vm: &mut VM, interrupt_type: InterruptType, flask_context: &mut FlaskContext, frame_buffer: &mut FrameBuffer) -> Result<(), String> {
    match interrupt_type {
        InterruptType::SysCall => Ok(syscall_handler(vm, flask_context, frame_buffer)?),
        InterruptType::Breakpoint => { Ok(()) },
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, TryFromPrimitive)]
enum SysCall {
    None = 0x00,
    Read = 0x01,
    Write = 0x02,
    RenderVRAM = 0x03,
    PollInputEvents = 0x04,
    // UpdateCursorState = 0x05,
    // GetMouseButtonState= 0x06,
}

fn syscall_handler(vm: &mut VM, flask_context: &mut FlaskContext, frame_buffer: &mut FrameBuffer) -> Result<(), String> {
    let syscall_id = vm.stack_pop().unwrap();
    let syscall = SysCall::try_from(syscall_id).unwrap();

    match syscall {
        SysCall::None => { },
        SysCall::Read => {
            // TODO:
        },
        SysCall::Write => {
            let size = vm.stack_pop().unwrap();
            let data_address = vm.stack_pop_address().unwrap();
            let _output_index = vm.stack_pop().unwrap();

            let mut data = vec![];
            for offset in 0..size as u16 {
                data.push(vm.peek_memory(data_address + offset).unwrap());
            }

            // TODO: use output_index
            flask::log(format!("{}", String::from_utf8(data).unwrap()).as_str());
        },
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
                        frame_buffer.set_pixel(column, line, duo_pixel & 0x0f);
                        frame_buffer.set_pixel(column + 1, line, (duo_pixel & 0xf0) >> 4);
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

            flask_context.render_frame_buffer(frame_buffer);
        },
        SysCall::PollInputEvents => {

            //TODO:
//            // Polling input events. If there are any key input press events pushed to the stack,
//            // we also push 0x01 to the stack, otherwise, pushing 0x00.
//            match flask_context.poll_sdl_input_event() {
//                Some(event) => {
//                    match input::process_sdl_event(&mut vm, &event)? {
//                        Some(_) => {
//                            vm.stack_push(0x01)?;
//                        }
//                        None => {
//                            vm.stack_push(0x00)?;
//                        }
//                    }
//                }
//                None => {
//                    vm.stack_push(0x00)?;
//                }
//            }
        },
    }

    Ok(())
}
