pub use maple::*;
use rand::Rng;
use render::mesh::Canvas;
use runtime::Runtime;

struct State {}
impl State {
    fn new() -> Self {
        Self {}
    }
}

impl runtime::State<Event> for State {
    fn update(&mut self, event: Event) {
        match event {
            Event::DeleteGraphic(canvas) => {
                // for mesh in &mut self.main_canvas.meshes {
                //     self.main_canvas.delete(&mesh);
                // }
            }
            Event::CreateGraphic(mut canvas) => {
                let mut rng = rand::thread_rng();
                let x_size = 500;
                let y_size = 500;
                for x in -x_size..x_size {
                    for y in -y_size..y_size {
                        let x = x as f32;
                        let y = y as f32;
                        let x_size = x_size as f32;
                        let y_size = y_size as f32;

                        let x_scale = 1.0 / x_size;
                        let y_scale = 1.0 / y_size;
                        let px = x_scale * (x + 0.5);
                        let py = y_scale * (y + 0.5);
                        let m = canvas.quad(px, py, x_scale, y_scale);

                        let r = rng.gen::<f32>(); // * x.abs() / x_size;
                        let g = rng.gen::<f32>(); // * y.abs() / y_size;
                        let b = rng.gen::<f32>(); // * (x.abs() + y.abs()) / (x_size + y_size);
                        canvas.color(&m, [r, g, b, 1.0]);
                    }
                }
            }
            Event::Tick => {}
        }
    }
}
#[derive(Debug)]
enum Event {
    Tick,
    CreateGraphic(Canvas),
    DeleteGraphic(Canvas),
}

fn run() {
    env_logger::init();
    let mut runtime = Runtime::new();
    futures::executor::block_on(runtime.new_display("main"));
    let state = State::new();
    runtime.start(state);
}

fn main() -> Result<(), std::io::Error> {
    run();
    Ok(())
}
