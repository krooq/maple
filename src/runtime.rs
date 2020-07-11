use crate::render::display::Display;
use std::time;
use winit::{event::Event, event_loop::EventLoop};

pub fn run() {
    env_logger::init();
    futures::executor::block_on(start());
}

/// Starts the application runtime taking over the executing thread on native platforms.
///
/// The runtime provides a simple method for running a graphical application,
/// if you need more complex behaviour you should create your own.
async fn start() {
    let event_loop = EventLoop::new();
    let gpu = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
    let mut displays = vec![Display::new(&event_loop, &gpu, "main").await];

    let mut last_update_inst = time::Instant::now();

    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            if last_update_inst.elapsed() > time::Duration::from_millis(20) {
                for display in displays.as_slice() {
                    display.request_redraw();
                }
                last_update_inst = time::Instant::now();
            }
        }
        Event::WindowEvent { event, window_id } => {
            for display in &mut displays[..] {
                display.send_event(&event, window_id, control_flow);
            }
        }
        Event::RedrawRequested(window_id) => {
            for display in &mut displays[..] {
                display.draw(window_id, Vec::new());
            }
        }
        _ => {}
    });
}
