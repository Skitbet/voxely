use camera::Camera;
use nalgebra::{Point3, Vector3};
use render::{chunk::VoxelChunk, Renderer};
use winit::{event::{ElementState, Event, KeyEvent, WindowEvent}, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowBuilder}};

pub mod camera;
pub mod render;

fn run(event_loop: EventLoop<()>, window: &Window) {
    let camera = Camera {
        eye: Point3::new(0.0, 5.0, 10.0),
        target: Point3::new(0.0, 0.0, 0.0),
        up: Vector3::y_axis().into_inner(),
        aspect_ratio: window.inner_size().width as f32 / window.inner_size().height as f32,
        fov: 45.0_f32.to_radians(),
        znear: 0.1,
        zfar: 100.0,
    };

    let mut renderer = futures::executor::block_on(Renderer::new(&window));

    let mut chunk = VoxelChunk::new(0, 0, 0);
    chunk.set_voxel(1, 1, 1, 1);
    chunk.set_voxel(2, 2, 2, 1);
    chunk.set_voxel(3, 3, 3, 1);
    let (vertex_buffer, index_buffer, index_count) = renderer.create_chunk_mesh(&chunk);
    
    println!("{:?} {:?} {}", vertex_buffer, index_buffer, index_count);

    let pipeline = renderer.create_pipeline();

    let _ = event_loop.run(move |event, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id: _,
            } => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            state: ElementState::Pressed,
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                    ..
                } => control_flow.exit(),
                WindowEvent::RedrawRequested => {
                    let view_proj = camera.build_view_proj_matrix();
                    renderer.update_camera(&view_proj);

                    renderer.render(&pipeline, &vertex_buffer, &index_buffer, index_count);
                },
                _ => {}
            },
            _ => {}
        }
    });
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    run(event_loop, &window);
}