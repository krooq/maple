use glsl_to_spirv;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use glsl_to_spirv::ShaderType;

    // Tell the build script to only run again if we change our source shaders
    println!("cargo:rerun-if-changed=src/render/shader");

    // Create destination path if necessary
    std::fs::create_dir_all("src/render/shader")?;

    for entry in std::fs::read_dir("src/render/shader")? {
        let entry = entry?;

        if entry.file_type()?.is_file() {
            let in_path = entry.path();

            // Support only vertex and fragment shaders currently
            let shader_type =
                in_path
                    .extension()
                    .and_then(|ext| match ext.to_string_lossy().as_ref() {
                        "vert" => Some(ShaderType::Vertex),
                        "frag" => Some(ShaderType::Fragment),
                        _ => None,
                    });

            if let Some(shader_type) = shader_type {
                use std::io::Read;

                let source = std::fs::read_to_string(&in_path)?;
                let mut compiled_file = glsl_to_spirv::compile(&source, shader_type)?;

                let mut compiled_bytes = Vec::new();
                compiled_file.read_to_end(&mut compiled_bytes)?;

                let out_path = format!(
                    "src/render/shader/{}.spv",
                    in_path.file_name().unwrap().to_string_lossy()
                );

                std::fs::write(&out_path, &compiled_bytes)?;
            }
        }
    }

    Ok(())
}
