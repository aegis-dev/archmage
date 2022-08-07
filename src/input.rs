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

use sdl2::event::Event;

use shard_vm::vm::VM;
use crate::memory::{CURSOR_POSITION_X, CURSOR_POSITION_Y, VIDEO_BUFFER_WIDTH, VIDEO_BUFFER_HEIGHT};

const CURSOR_SENSITIVITY: f32 = 1.0;

#[allow(unused_variables)]
pub fn process_sdl_event(vm: &mut VM, event:& Event) -> Result<Option<()>, String> {
    match event {
        Event::KeyDown {
            timestamp,
            window_id,
            keycode,
            scancode,
            keymod,
            repeat
        } => {
            let keycode_value = keycode.unwrap();
            if keycode_value as u32 <= u8::MAX as u32 {
                vm.stack_push(keycode_value as u8)?;
                vm.stack_push(0x01)?;
                return Ok(Some(()))
            }
            Ok(None)
        }
        Event::KeyUp {
            timestamp,
            window_id,
            keycode,
            scancode,
            keymod,
            repeat
        } => {
            let keycode_value = keycode.unwrap();
            if keycode_value as u32 <= u8::MAX as u32 {
                vm.stack_push(keycode_value as u8)?;
                vm.stack_push(0x00)?;
                return Ok(Some(()))
            }
            Ok(None)
        }
        Event::MouseMotion {
            timestamp,
            window_id,
            which,
            mousestate,
            x,
            y,
            xrel,
            yrel,
        } => {
            let mut cursor_x: f32 = vm.peek_memory(CURSOR_POSITION_X).unwrap() as f32;
            let mut cursor_y: f32 = vm.peek_memory(CURSOR_POSITION_Y).unwrap() as f32;

            let adjusted_x_rel = *xrel as f32 * CURSOR_SENSITIVITY;
            let adjusted_y_rel = *yrel as f32 * CURSOR_SENSITIVITY;

            cursor_x += adjusted_x_rel;
            if cursor_x > VIDEO_BUFFER_WIDTH as f32 {
                cursor_x = VIDEO_BUFFER_WIDTH as f32;
            } else if cursor_x < 0.0 {
                cursor_x = 0.0;
            }
            cursor_y -= adjusted_y_rel; // subtract to invert y value
            if cursor_y > VIDEO_BUFFER_HEIGHT as f32 {
                cursor_y = VIDEO_BUFFER_HEIGHT as f32;
            } else if cursor_y < 0.0 {
                cursor_y = 0.0;
            }

            let memory = vm.get_memory_mut();
            memory.write_u8(CURSOR_POSITION_X, cursor_x as u8)?;
            memory.write_u8(CURSOR_POSITION_Y, cursor_y as u8)?;

            Ok(None)
        }
        Event::MouseButtonDown {
            timestamp,
            window_id,
            which,
            mouse_btn,
            clicks,
            x,
            y,
        } => {
           // TODO: set mouse state in memory

            Ok(None)
        }
        Event::MouseButtonUp {
            timestamp,
            window_id,
            which,
            mouse_btn,
            clicks,
            x,
            y,
        } => {
            // TODO: set mouse state in memory

            Ok(None)
        }
        Event::Quit {
            timestamp
        } => {
            // TODO: how to shutdown machine?
            Ok(None)
        }
        _ => {
            Ok(None)
        }
    }
}