use super::mesh::Canvas;
use crate::render::renderer::Renderer;
use winit::dpi::PhysicalSize;
use winit::{
    event::{self, WindowEvent},
    event_loop::ControlFlow,
};

/// A [`Display`] combines a presentable window with a graphics renderer.
///
/// [`Display`]: struct.Display.html
pub struct Display {
    window: winit::window::Window,
    surface: wgpu::Surface,
    renderer: Renderer,
    pub canvas: Canvas,
}

/// [`Display`]: struct.Display.html
impl Display {
    pub async fn new<E, T: Into<String>>(
        event_loop: &winit::event_loop::EventLoopWindowTarget<E>,
        instance: &wgpu::Instance,
        title: T,
    ) -> Self {
        let window = winit::window::WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .unwrap();
        let size = window.inner_size();

        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let trace_dir = std::env::var("WGPU_TRACE");
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    shader_validation: true,
                    ..Default::default()
                },
                trace_dir.ok().as_ref().map(std::path::Path::new),
            )
            .await
            .unwrap();

        let renderer = Renderer::new(&surface, device, queue, size.width, size.height);

        let canvas = Canvas::new();

        Self {
            window,
            surface,
            renderer,
            canvas,
        }
    }

    pub fn resize(&mut self, size: &PhysicalSize<u32>) {
        log::info!("Resizing to {:?}", size);
        self.renderer.resize(size.width, size.height);
    }

    pub fn draw(&mut self, window_id: winit::window::WindowId) {
        if window_id == self.window.id() {
            self.renderer.draw_frame(
                &self.surface,
                self.canvas.vertices.as_slice(),
                self.canvas.indices.as_slice(),
                self.canvas.transforms.as_slice(),
            );
        }
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn send_event(
        &mut self,
        event: &WindowEvent,
        window_id: winit::window::WindowId,
        control_flow: &mut ControlFlow,
    ) {
        if window_id == self.window.id() {
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
}
