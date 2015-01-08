// See LICENSE file for copyright and license details.

use core_types::{ZInt};
use visualizer_types::{VertexCoord};
use zgl::{Zgl, Vbo, MeshRenderMode};
use shader::{Shader};

pub struct Mesh {
    vertex_coords_vbo: Vbo,
    length: ZInt,
    mode: MeshRenderMode,
}

impl Mesh {
    pub fn new(zgl: &Zgl, data: &[VertexCoord]) -> Mesh {
        let length = data.len() as ZInt;
        let vertex_coords_vbo = Vbo::from_data(zgl, data);
        Mesh {
            vertex_coords_vbo: vertex_coords_vbo,
            length: length,
            mode: MeshRenderMode::Triangles,
        }
    }

    pub fn draw(&self, zgl: &Zgl, shader: &Shader) {
        self.vertex_coords_vbo.bind(zgl);
        shader.enable_attr(zgl, &shader.get_position_attr_id(), 3);
        zgl.draw_arrays(&self.mode, self.length);
    }
}

// vim: set tabstop=4 shiftwidth=4 softtabstop=4 expandtab: