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

extern crate gl;
extern crate sdl2;

pub mod shaders;
pub mod mesh;
pub mod texture;
pub mod renderer;

use crate::mesh::Mesh;
use crate::texture::{Texture, ImageMode};

fn main() {
    let window_width: u32 = 800;
    let window_height: u32 = 600;

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("archmage", window_width, window_height)
        .opengl()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    // Set clear color
    unsafe {
        gl::Viewport(0, 0, window_width as i32, window_height as i32);
    }

    // Load shaders
    let shader = {
        use std::ffi::CString;
        shaders::ShaderProgram::load_shaders(
            &CString::new(include_str!("shaders/screen_shader.vert")).unwrap(),
            &CString::new(include_str!("shaders/screen_shader.frag")).unwrap(),
        )
    };

    let renderer = renderer::Renderer::new(shader);
    renderer.set_clear_color(0.0, 0.0, 0.0, 0.0);

    let vertices = vec![
         -1.0, -1.0, 0.0, // bot left
          1.0,  1.0, 0.0, // top right
          1.0, -1.0, 0.0, // top left
         -1.0,  1.0, 0.0  // bot right
    ];
    let texture_coords = vec![
        0.0, 0.0,
        1.0, 1.0,
        0.0, 1.0,
        1.0, 0.0,
    ];
    let indices = vec![
        0, 1, 3,
        0, 2, 1
    ];

    let frame_buffer_quad = Mesh::from_data(&vertices, &texture_coords, &indices);

    let mut rendered_frame_data: Vec<u8> = vec![123; (window_width * window_height * 3) as usize];
    let mut frame_data_texture = Texture::from_data(&rendered_frame_data, window_width, window_height, ImageMode::RGB);

    let mut event_pump = sdl.event_pump().unwrap();
    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main_loop,
                _ => {},
            }
        }

        // loop on_update

        // update render texture
        frame_data_texture.update_texture_data(&rendered_frame_data, window_width, window_height, ImageMode::RGB);

        renderer.clear_buffer();

        renderer.begin_rendering();
        renderer.render(&frame_buffer_quad, &frame_data_texture);
        renderer.end_rendering();

        window.gl_swap_window();
    }
}
