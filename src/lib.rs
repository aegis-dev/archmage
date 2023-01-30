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

mod memory;
mod interrupts;
mod palette;
mod video_mode;

use std::cell::RefCell;
use std::rc::Rc;
use memory::{CALL_STACK_START, CALL_STACK_SIZE};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::{Closure, wasm_bindgen};

use js_sys::{Reflect, Uint8Array};

use shard_vm::vm::{
    VM, ExecutionStatus, InterruptType
};

use flask::flask_context::FlaskContext;
use flask::frame_buffer::FrameBuffer;

use crate::memory::{
    MachineMemory, VIDEO_BUFFER_WIDTH, VIDEO_BUFFER_HEIGHT,
    VIDEO_MODE, VIDEO_BUFFER_START, VIDEO_BUFFER_SIZE,
    STACK_START, STACK_SIZE, CURSOR_POSITION_Y, CURSOR_POSITION_X
};


#[wasm_bindgen(start)]
pub fn start() {
    if let Err(err) = run_machine() {
        flask::log(format!("{}", err).as_str());
    }
}

fn print_sys_info() {
    flask::log("--------------------- CPU ----------------------");
    flask::log("8 bit execution mode");
    flask::log("16 bit addressing");
    flask::log("Powered by shard_lang - toy assembly language and VM");
    flask::log("");
    flask::log("----------------- Screen size ------------------");
    flask::log(format!("VIDEO_BUFFER_WIDTH    {0}", VIDEO_BUFFER_WIDTH).as_str());
    flask::log(format!("VIDEO_BUFFER_HEIGHT   {0}", VIDEO_BUFFER_HEIGHT).as_str());
    flask::log(format!("VIDEO_BUFFER_SIZE     {0}", VIDEO_BUFFER_SIZE).as_str());
    flask::log("");
    flask::log("---------------- Memory Layout -----------------");
    flask::log(format!("STACK_START           0x{0:x} - 0x{1:x}", STACK_START, STACK_START + STACK_SIZE - 1).as_str());
    flask::log(format!("CALL_STACK_START      0x{0:x} - 0x{1:x}", CALL_STACK_START, CALL_STACK_START + CALL_STACK_SIZE - 1).as_str());
    flask::log(format!("VIDEO_BUFFER_START    0x{0:x} - 0x{1:x}", VIDEO_BUFFER_START, VIDEO_BUFFER_START + VIDEO_BUFFER_SIZE - 1).as_str());
    flask::log(format!("VIDEO_MODE            0x{0:x}", VIDEO_MODE).as_str());
    flask::log(format!("CURSOR_POSITION_Y     0x{0:x}", CURSOR_POSITION_Y).as_str());
    flask::log(format!("CURSOR_POSITION_X     0x{0:x}", CURSOR_POSITION_X).as_str());
    flask::log("");
    flask::log("------------------------------------------------");
}

pub fn run_machine() -> Result<(), String> {
    print_sys_info();

    let window = web_sys::window().unwrap();
    let kernel_bytes = match Reflect::get(&window, &JsValue::from_str("kernel")) {
        Ok(value) => {
            value.unchecked_ref::<Uint8Array>().to_vec()
        },
        _ => return Err(String::from("Failed to load kernel"))
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
        false,
        palette::archmage_palette()
    ) {
        Ok(context) => context,
        Err(err) => {
            return Err(String::from(format!("Failed to create window context: {}", err)));
        }
    };

    let mut frame_buffer = FrameBuffer::new(
        flask_context.get_gl_context(),
        VIDEO_BUFFER_WIDTH as u32,
        VIDEO_BUFFER_HEIGHT as u32
    );

    let mut vm = VM::new_with_custom_memory(memory);

//    loop {
//        let status = vm.execute_instruction()?;
//        let sys_call = match status {
//            ExecutionStatus::Continue => continue,
//            ExecutionStatus::Done => return Ok(()),
//            ExecutionStatus::SysCall => {
//                interrupts::interrupt_handler(&mut vm, InterruptType::SysCall, &mut flask_context, &mut frame_buffer);
//            },
//            ExecutionStatus::Breakpoint => {
//                interrupts::interrupt_handler(&mut vm, InterruptType::Breakpoint, &mut flask_context, &mut frame_buffer);
//            },
//        };
//    }

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if let Err(err) = update(&mut vm, &mut flask_context, &mut frame_buffer) {
            flask::log(format!("{}", err).as_str());
            return;
        }

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

fn update(vm: &mut VM, flask_context: &mut FlaskContext, frame_buffer: &mut FrameBuffer) -> Result<(), String> {
    match vm.execute_instruction()? {
        ExecutionStatus::Continue => { },
        ExecutionStatus::Done => return Err(String::from("Done")),
        ExecutionStatus::SysCall => {
            interrupts::interrupt_handler(vm, InterruptType::SysCall, flask_context, frame_buffer)?;
        },
        ExecutionStatus::Breakpoint => {
            interrupts::interrupt_handler(vm, InterruptType::Breakpoint, flask_context, frame_buffer)?;
        },
    };

    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    FlaskContext::get_window().request_animation_frame(f.as_ref().unchecked_ref()).expect("Failed to request animation frame");
}
