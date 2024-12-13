use wgpu::{util::DeviceExt, Buffer};

use super::Renderer;

pub struct Cube {
    vertices: Vec<f32>,
    indices: Vec<u32>,
}

impl Cube {
    pub fn new() -> Self {
        let vertices = vec![
            // positions (x, y, z)
            -0.5, -0.5, -0.5,  // Front bottom left
            0.5, -0.5, -0.5,   // Front bottom right
            0.5, 0.5, -0.5,    // Front top right
            -0.5, 0.5, -0.5,   // Front top left
            -0.5, -0.5, 0.5,   // Back bottom left
            0.5, -0.5, 0.5,    // Back bottom right
            0.5, 0.5, 0.5,     // Back top right
            -0.5, 0.5, 0.5,    // Back top left
        ];

        let indices = vec![
            0, 1, 2, 0, 2, 3, // front face
            4, 5, 6, 4, 6, 7, // back face
            0, 1, 5, 0, 5, 4, // bottom face
            2, 3, 7, 2, 7, 6, // top face
            0, 3, 7, 0, 7, 4, // left face
            1, 2, 6, 1, 6, 5, // right face
        ];

        Cube { vertices, indices }
    }
}

impl<'a> Renderer<'a> {
    pub fn create_mesh(&self) -> (Buffer, Buffer) {
        let cube = Cube::new();
        
        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&cube.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&cube.indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        (vertex_buffer, index_buffer)
    }
}
