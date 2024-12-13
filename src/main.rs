use render::{chunk::VoxelChunk, Renderer};
use winit::{event::{ElementState, Event, KeyEvent, WindowEvent}, event_loop::EventLoop, keyboard::{KeyCode, PhysicalKey}, window::{Window, WindowBuilder}};

pub mod render;

fn run(event_loop: EventLoop<()>, window: &Window) {
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