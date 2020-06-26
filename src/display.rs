use crate::pipeline::Pipeline;
use winit::dpi::PhysicalSize;
use winit::{
    event::{self, WindowEvent},
    event_loop::ControlFlow,
};
/// [`Display`]: ./struct.Display.html
pub struct Display {
    window: winit::window::Window,
    surface: wgpu::Surface,
    pipeline: Pipeline,
}

impl Display {
    pub async fn new<T: Into<String>>(
        event_loop: &winit::event_loop::EventLoopWindowTarget<()>,
        instance: &wgpu::Instance,
        title: T,
    ) -> Self {
        let window = winit::window::WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .unwrap();
        let surface = unsafe { instance.create_surface(&window) };
        let size = window.inner_size();
        let adapter = instance
            .request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::Default,
                    compatible_surface: Some(&surface),
                },
                wgpu::UnsafeExtensions::disallow(),
                wgpu::BackendBit::PRIMARY,
            )
            .await
            .unwrap();

        let trace_dir = std::env::var("WGPU_TRACE");
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor::default(),
                trace_dir.ok().as_ref().map(std::path::Path::new),
            )
            .await
            .unwrap();

        let pipeline = Pipeline::new(&surface, device, queue, size.width, size.height);

        Self {
            window,
            surface,
            pipeline,
        }
    }

    pub fn resize(&mut self, size: &PhysicalSize<u32>) {
        log::info!("Resizing to {:?}", size);
        self.pipeline.resize(size.width, size.height);
    }

    pub fn draw(&mut self) {
        self.pipeline.render_next_frame(&self.surface);
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn send_event(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) {
        match event {
            WindowEvent::Resized(size) => {
                self.resize(&size);
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
            // TODO: change the way events work
            | WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    }
}
