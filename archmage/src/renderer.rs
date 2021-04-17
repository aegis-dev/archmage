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

use gl;

use crate::shaders::ShaderProgram;
use crate::mesh::Mesh;
use crate::texture::Texture;
use std::ptr::null;

pub struct Renderer {
    shader: ShaderProgram,
}

impl Renderer {
    pub fn new(shader: ShaderProgram) -> Renderer {
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        Renderer { shader }
    }

    pub fn clear_buffer(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn begin_rendering(&self) {
        self.shader.enable();
    }

    pub fn end_rendering(&self) {
        self.shader.disable();
    }

    pub fn set_clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    pub fn render(&self, mesh: &Mesh, texture: &Texture) {
        unsafe {
            gl::BindVertexArray(mesh.vao_id());
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.texture_id());

            gl::DrawElements(gl::TRIANGLES, mesh.indices_count(), gl::UNSIGNED_INT, null());

            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
            gl::BindVertexArray(0);
        }
    }
}