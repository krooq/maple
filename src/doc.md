- instance = wgpu::Instance::new();
- surface   = instance.create_surface       instance &window
- adapter   = instance.request_adapter      instance &surface
- device    = adapter.request_device        adapter
- queue     = adapter.request_device        adapter


- device.create_shader_module               "vertex_shader.spv"
- device.create_shader_module               "fragment_shader.spv"