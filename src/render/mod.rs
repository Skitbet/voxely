
use chunk::VoxelChunk;
use nalgebra::Matrix;
use wgpu::{util::DeviceExt, Buffer};
use winit::window::Window;

pub mod chunk;
pub mod cube;

pub struct Renderer<'a> {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'a>,
    pub config: wgpu::SurfaceConfiguration,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: Option<wgpu::BindGroup>,
} 

impl<'a> Renderer<'a> {
    pub async fn new(window: &'a &Window) -> Renderer<'a> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { 
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor{
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: Default::default(),
            },
            None,
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Opaque,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: &[0u8; 64],
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Renderer {
            device,
            queue,
            surface,
            config,
            camera_buffer,
            camera_bind_group: None,
        }
    }

pub fn update_camera(&self, view_proj: &Matrix<f32, nalgebra::Const<4>, nalgebra::Const<4>, nalgebra::ArrayStorage<f32, 4, 4>>) {
    // convert the matrix to a [f32; 16]
    let view_proj_array: [f32; 16] = {
        let slice = view_proj.as_slice(); // gives a slice of 16 f32 values
        let mut array = [0.0; 16]; // init array to store the values
        array.copy_from_slice(slice); // copy  slice values into the array
        array
    };

    // Write to the buffer
    self.queue.write_buffer(
        &self.camera_buffer,
        0,
        bytemuck::cast_slice(&view_proj_array),
    );
}

    pub fn create_chunk_mesh(&mut self, chunk: &VoxelChunk) -> (wgpu::Buffer, wgpu::Buffer, u32) {
        let (vertices, indices) = chunk.generate_mesh();
        println!("{:?} {:?}", vertices, indices);

        let vertex_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Chunk Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Chunk Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        (vertex_buffer, index_buffer, indices.len() as u32)
    }

    pub fn create_pipeline(&mut self) -> wgpu::RenderPipeline {
        let surface_format = self.config.format;

        // Load shader
        let shader = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/shader.wgsl").into()),
        });

        // create bind group layout for camera
        let camera_bind_group_layout = self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Camera Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        // create bind group for camera buffer
        self.camera_bind_group = Some(self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: self.camera_buffer.as_entire_binding(),
            }],
        }));


        // Define the pipeline layout
        let pipeline_layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create render pipeline
        self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                module: &shader,
                entry_point: Some("vs_main"), // Entry function in WGSL
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 3 * std::mem::size_of::<f32>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x3,
                            offset: 0,
                            shader_location: 0,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                module: &shader,
                entry_point: Some("fs_main"), // Entry function in WGSL
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        })
    }

    pub fn render(
        &self,
        pipeline: &wgpu::RenderPipeline,
        vertex_buffer: &wgpu::Buffer,
        index_buffer: &wgpu::Buffer,
        index_count: u32,
    ) {
        println!("Rendering frame...");

        let frame = self
            .surface
            .get_current_texture()
            .expect("Failed to acquire next swap chain texture");
        
        println!("Acquired frame texture");
        
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        
            render_pass.set_pipeline(pipeline);
            render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..index_count, 0, 0..1);
        }
        
        self.queue.submit(Some(encoder.finish()));
        frame.present();
        println!("Frame presented.");
    }
}