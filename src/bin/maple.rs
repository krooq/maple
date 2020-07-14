pub use maple::*;
use rand::Rng;
use runtime::Runtime;

fn run() {
    env_logger::init();
    let mut runtime = Runtime::new();
    let display = futures::executor::block_on(runtime.new_display("main"));

    let canvas = &mut display.canvas;

    let mut rng = rand::thread_rng();
    let x_size = 10;
    let y_size = 10;
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
    runtime.start();
}

fn main() -> Result<(), std::io::Error> {
    run();
    Ok(())
}
