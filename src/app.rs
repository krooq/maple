use crate::window::Window;

use log::info;
use std::time;
use winit::{
    dpi::PhysicalSize,
    event::{self, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

struct App {
    gpu: Option<wgpu::Instance>,
    windows: Vec<Window>,
}

impl App {
    async fn new() -> Self {
        Self {
            gpu: Some(gpu),
            windows: Vec::new(),
        }
    }
    async fn run() {}

    pub fn create_window(&mut self, title: &str) {
        self.windows.pust(Window::new(title));
    }

    fn exit(control_flow: &mut ControlFlow) {
        *control_flow = ControlFlow::Exit;
    }
}

pub fn run(title: &str) {
    let event_loop = EventLoop::new();
    let builder = winit::window::WindowBuilder::new().with_title(title);
    let window = builder.build(&event_loop).unwrap();
    env_logger::init();
    futures::executor::block_on(run_async(event_loop, window));
}

async fn run_async(event_loop: EventLoop<()>, window: Window) {
    info!("Initializing the gpu...");
    let mut app = App::new().await;
    app.create_window("app");
    let mut last_update_inst = time::Instant::now();

    log::info!("Entering event loop...");
    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            if last_update_inst.elapsed() > time::Duration::from_millis(20) {
                window.request_redraw();
                last_update_inst = time::Instant::now();
            }
        }
        Event::WindowEvent {
            event,
            window_id: _,
        } => match event {
            WindowEvent::Resized(size) => {
                app.resize(&size);
            }
            WindowEvent::KeyboardInput {
                input:
                    event::KeyboardInput {
                        virtual_keycode: Some(event::VirtualKeyCode::Escape),
                        state: event::ElementState::Pressed,
                        ..
                    },
                ..
            }
            | WindowEvent::CloseRequested => {
                App::exit(control_flow);
            }
            _ => {}
        },
        Event::RedrawRequested(_) => {
            app.gpu.render();
        }
        _ => {}
    });
}
