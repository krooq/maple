use crate::render::display::Display;
use std::time;
use winit::{event::Event, event_loop::EventLoop};

pub trait State<E: 'static> {
    fn update(&mut self, event: E);
}

pub struct Runtime<E: 'static> {
    event_loop: winit::event_loop::EventLoop<E>,
    pub event_loop_proxy: winit::event_loop::EventLoopProxy<E>,
    gpu: wgpu::Instance,
    displays: Vec<Display>,
}

impl<E> Runtime<E>
where
    E: 'static,
{
    pub fn new() -> Self {
        let event_loop = EventLoop::<E>::with_user_event();
        let event_loop_proxy = event_loop.create_proxy();
        let gpu = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let displays = Vec::new();
        Self {
            event_loop,
            event_loop_proxy,
            gpu,
            displays,
        }
    }

    // pub fn send_event<E>(&self, event: E) {
    //     self.event_loop_proxy.send_event(event);
    // }

    pub async fn new_display<T: Into<String>>(&mut self, title: T) {
        self.displays
            .push(Display::new(&self.event_loop, &self.gpu, title).await);
    }

    /// Starts the application runtime taking over the executing thread on native platforms.
    ///
    /// The runtime provides a simple method for running a graphical application,
    /// if you need more complex behaviour you should create your own.
    pub fn start<S: State<E> + 'static>(self, mut state: S) {
        let mut displays = self.displays;
        // let state = &mut state;
        let mut last_update_inst = time::Instant::now();
        self.event_loop
            .run(move |event, _, control_flow| match event {
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
                        display.draw(window_id);
                    }
                }
                Event::UserEvent(event) => {
                    state.update(event);
                }
                _ => {}
            });
    }
}
