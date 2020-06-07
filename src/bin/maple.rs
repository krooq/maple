pub use maple::*;

fn main() -> Result<(), std::io::Error> {
    // rgx_example::run()
    // wgpu_shadow_example::main();
    // Ok(())
    // wgpu_hello_triangle_example::init()
    app::run::<crate::wgpu_cube_example::App>("cube");
    Ok(())
}
