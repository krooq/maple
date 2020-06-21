use crate::pipeline::Pipeline;
use winit::dpi::PhysicalSize;
use winit::{
    event::{self, WindowEvent},
    event_loop::ControlFlow,
};

pub struct Display {
    window: winit::window::Window,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    swap_chain_descriptor: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
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

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    extensions: wgpu::Extensions::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                std::env::var("WGPU_TRACE").ok().as_ref().map(std::path::Path::new),
            )
            .await
            .unwrap();

        let swap_chain_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);
        let pipeline = Pipeline::new(&device, swap_chain_descriptor.format);

        Self {
            window,
            surface,
            device,
            queue,
            swap_chain_descriptor,
            swap_chain,
            pipeline,
        }
    }

    pub fn submit<I: IntoIterator<Item = wgpu::CommandBuffer>>(&self, command_buffers: I) {
        self.queue.submit(command_buffers);
    }

    pub fn resize(&mut self, size: &PhysicalSize<u32>) {
        log::info!("Resizing to {:?}", size);
        self.swap_chain_descriptor.width = size.width;
        self.swap_chain_descriptor.height = size.height;
    }

    pub fn draw(&mut self) {
        let frame = match self.swap_chain.get_next_frame() {
            Ok(frame) => frame,
            Err(_) => {
                self.swap_chain = self
                    .device
                    .create_swap_chain(&self.surface, &self.swap_chain_descriptor);
                self.swap_chain
                    .get_next_frame()
                    .expect("Failed to acquire next swap chain texture!")
            }
        };
        let command_buffer = self.pipeline.render(&self.device, &frame);
        self.submit(Some(command_buffer));
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
