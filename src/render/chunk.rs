use nalgebra::{Vector3, Matrix4};

const CHUNK_SIZE: usize = 16;

pub struct VoxelChunk {
    data: [[[u8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
    position: Vector3<i32>,
}

impl VoxelChunk {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        VoxelChunk {
            data: [[[0; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
            position: Vector3::new(x, y, z),
        }
    }

    pub fn generate_mesh(&self) -> (Vec<f32>, Vec<u32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut index_offset = 0;

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if self.data[x][y][z] == 0 {
                        continue; // Skip empty voxels
                    }

                    // Generate cube vertices and indices for the current voxel
                    let cube_vertices = vec![
                        // Adjust positions based on the voxel's location in the chunk
                        x as f32, y as f32, z as f32,
                        (x + 1) as f32, y as f32, z as f32,
                        (x + 1) as f32, (y + 1) as f32, z as f32,
                        x as f32, (y + 1) as f32, z as f32,
                        x as f32, y as f32, (z + 1) as f32,
                        (x + 1) as f32, y as f32, (z + 1) as f32,
                        (x + 1) as f32, (y + 1) as f32, (z + 1) as f32,
                        x as f32, (y + 1) as f32, (z + 1) as f32,
                    ];
                    
                    vertices.extend(cube_vertices);

                    // Generate cube indices
                    let cube_indices = vec![
                        0, 1, 2, 0, 2, 3, // Front face
                        4, 5, 6, 4, 6, 7, // Back face
                        0, 1, 5, 0, 5, 4, // Bottom face
                        2, 3, 7, 2, 7, 6, // Top face
                        0, 3, 7, 0, 7, 4, // Left face
                        1, 2, 6, 1, 6, 5, // Right face
                    ].iter().map(|i| i + index_offset).collect::<Vec<_>>();
                    
                    indices.extend(cube_indices);

                    index_offset += 8; // Each cube adds 8 vertices
                }
            }
        }

        (vertices, indices)
    }
}
