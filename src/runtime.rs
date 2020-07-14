use crate::render::display::Display;
use std::time;
use winit::{event::Event, event_loop::EventLoop};

pub struct Runtime {
    event_loop: EventLoop<()>,
    gpu: wgpu::Instance,
    displays: Vec<Display>,
}

impl Runtime {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let gpu = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let displays = Vec::new();
        Self {
            event_loop,
            gpu,
            displays,
        }
    }

    pub async fn new_display<T: Into<String>>(&mut self, title: T) -> &mut Display {
        self.displays
            .push(Display::new(&self.event_loop, &self.gpu, title).await);
        let display = self.displays.last_mut();
        display.unwrap()
    }

    /// Starts the application runtime taking over the executing thread on native platforms.
    ///
    /// The runtime provides a simple method for running a graphical application,
    /// if you need more complex behaviour you should create your own.
    pub fn start(self) {
        let mut displays = self.displays;
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
                _ => {}
            });
    }
}

// /// Starts the application runtime taking over the executing thread on native platforms.
// ///
// /// The runtime provides a simple method for running a graphical application,
// /// if you need more complex behaviour you should create your own.
// async fn start() {
//     let event_loop = EventLoop::new();
//     let gpu = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
//     let mut displays = vec![Display::new(&event_loop, &gpu, "main").await];

//     let mut last_update_inst = time::Instant::now();

//     let canvas = &mut displays.get_mut(0).unwrap().canvas;

//     let x_size = 500;
//     let y_size = 500;
//     for x in -x_size..x_size {
//         for y in -y_size..y_size {
//             let x = x as f32;
//             let y = y as f32;
//             let x_size = x_size as f32;
//             let y_size = y_size as f32;

//             let x_scale = 1.0 / x_size;
//             let y_scale = 1.0 / y_size;
//             let px = x_scale * (x + 0.5);
//             let py = y_scale * (y + 0.5);
//             let m = canvas.quad(px, py, x_scale, y_scale);

//             let r = x.abs() / x_size;
//             let g = y.abs() / y_size;
//             let b = (x.abs() + y.abs()) / (x_size + y_size);
//             canvas.color(&m, [r, g, b, 1.0]);
//         }
//     }

//     event_loop.run(move |event, _, control_flow| match event {
//         Event::MainEventsCleared => {
//             if last_update_inst.elapsed() > time::Duration::from_millis(20) {
//                 for display in displays.as_slice() {
//                     display.request_redraw();
//                 }
//                 last_update_inst = time::Instant::now();
//             }
//         }
//         Event::WindowEvent { event, window_id } => {
//             for display in &mut displays[..] {
//                 display.send_event(&event, window_id, control_flow);
//             }
//         }
//         Event::RedrawRequested(window_id) => {
//             for display in &mut displays[..] {
//                 display.draw(window_id);
//             }
//         }
//         _ => {}
//     });
// }
