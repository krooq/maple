use crate::gpu;
use log::{info, warn};
use raw_window_handle::HasRawWindowHandle;
use std::time;
use winit::{
    dpi::PhysicalSize,
    event::{self, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

#[rustfmt::skip]
#[allow(unused)]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub trait App: 'static + Sized {
    fn init(
        sc_desc: &wgpu::SwapChainDescriptor,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> (Self, Option<wgpu::CommandBuffer>);
    fn resize(
        &mut self,
        sc_desc: &wgpu::SwapChainDescriptor,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    );
    fn update(&mut self, event: WindowEvent);
    fn render(
        &mut self,
        frame: &wgpu::SwapChainTexture,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> wgpu::CommandBuffer;
}

pub fn run<A: App>(title: &str) {
    let event_loop = EventLoop::new();
    let builder = winit::window::WindowBuilder::new().with_title(title);
    let window = builder.build(&event_loop).unwrap();
    env_logger::init();
    futures::executor::block_on(run_async::<A>(event_loop, window));
}

async fn run_async<A: App>(event_loop: EventLoop<()>, window: Window) {
    info!("Initializing the gpu...");
    let mut gpu = gpu::GPU::new(&window).await;

    log::info!("Initializing the app...");
    let (mut app, init_command_buf) = A::init(&gpu.swap_chain_descriptor, &gpu.device, &gpu.queue);
    if init_command_buf.is_some() {
        gpu.submit(init_command_buf);
    }

    let mut last_update_inst = time::Instant::now();

    log::info!("Entering event loop...");
    event_loop.run(move |event, _, control_flow| {
        // let _ = (&instance, &adapter); // force ownership by the closure
        // *control_flow = if cfg!(feature = "metal-auto-capture") {
        //     ControlFlow::Exit
        // } else {
        //     ControlFlow::WaitUntil(time::Instant::now() + time::Duration::from_millis(10))
        // };
        match event {
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
                    log::info!("Resizing to {:?}", size);
                    gpu.resize(size);
                    app.resize(&gpu.swap_chain_descriptor, &gpu.device, &gpu.queue);
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
                    *control_flow = ControlFlow::Exit;
                }
                _ => {
                    app.update(event);
                }
            },
            Event::RedrawRequested(_) => {
                let frame = gpu.render();
                let command_buffer = app.render(&frame.output, &gpu.device, &gpu.queue);
                gpu.submit(Some(command_buffer));
            }
            _ => {}
        }
    });
}
