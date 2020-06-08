use crate::pipeline::Pipeline;
use winit::{dpi::PhysicalSize, window::Window};

pub struct GPU {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    swap_chain_descriptor: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    pipeline: Pipeline,
}

impl GPU {
    pub async fn new(window: &Window) -> GPU {
        let instance = wgpu::Instance::new();
        let size = window.inner_size();
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::Default,
                    compatible_surface: Some(&surface),
                },
                wgpu::BackendBit::PRIMARY,
            )
            .await
            .unwrap();

        let trace_dir = std::env::var("WGPU_TRACE");
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    extensions: wgpu::Extensions::empty(),
                    limits: wgpu::Limits::default(),
                },
                trace_dir.ok().as_ref().map(std::path::Path::new),
            )
            .await
            .unwrap();

        let swap_chain_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            // TODO: Allow srgb unconditionally
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);
        let pipeline = Pipeline::new(&device, swap_chain_descriptor.format);

        GPU {
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

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.swap_chain_descriptor.width = size.width;
        self.swap_chain_descriptor.height = size.height;
    }

    pub fn render(&mut self) {
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
}

pub fn create_vertex_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
    let spv = &wgpu::read_spirv(std::io::Cursor::new(
        &include_bytes!("shader/shader.vert.spv")[..],
    ))
    .expect("Read shader as SPIR-V");
    device.create_shader_module(&spv)
}

pub fn create_fragment_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
    let spv = &wgpu::read_spirv(std::io::Cursor::new(
        &include_bytes!("shader/shader.frag.spv")[..],
    ))
    .expect("Read shader as SPIR-V");
    device.create_shader_module(&spv)
}
