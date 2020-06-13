use crate::gpu::RenderContext;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;

pub struct Window {
    title: String,
    event_loop: EventLoop<()>,
    window: winit::window::Window,
    render_context: RenderContext,
}

impl Window {
    fn new(title: &str) -> Self {
        env_logger::init();
        let event_loop = EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .unwrap();
        let render_context: RenderContext::new(&window);
        Self {
            title,
            event_loop,
            window,
            render_context,
        }
    }

    fn resize(&mut self, size: &PhysicalSize<u32>) {
        log::info!("Resizing to {:?}", size);
        self.render_context.resize(size);
    }
}
