use crate::display::Display;
use std::time;
use winit::{event::Event, event_loop::EventLoop};

pub fn run() {
    env_logger::init();
    futures::executor::block_on(start());
}

async fn start() {
    let event_loop = EventLoop::new();
    let gpu = wgpu::Instance::new();
    let mut displays = vec![Display::new(&event_loop, &gpu, "main").await];

    let mut last_update_inst = time::Instant::now();
    log::info!("Entering event loop...");
    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            if last_update_inst.elapsed() > time::Duration::from_millis(20) {
                for display in displays.as_slice() {
                    display.request_redraw();
                }
                last_update_inst = time::Instant::now();
            }
        }
        Event::WindowEvent {
            event,
            window_id: _,
        } => {
            for display in displays.as_mut_slice() {
                display.send_event(&event, control_flow);
            }
        }
        Event::RedrawRequested(_) => {
            for display in displays.as_mut_slice() {
                display.draw();
            }
        }
        _ => {}
    });
}