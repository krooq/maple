- instance = wgpu::Instance::new();
- surface   = instance.create_surface       instance &window
- adapter   = instance.request_adapter      instance &surface
- device    = adapter.request_device        adapter
- queue     = adapter.request_device        adapter


- device.create_shader_module               "vertex_shader.spv"
- device.create_shader_module               "fragment_shader.spv"



## vertex buffer 
- define a vertex (use bytemuck to be able to turn it into &[u8] i.e. byte array)
- create VertexBufferDescriptor in a static method e.g. Vertex::desc
- call Vertex::desc to put it in VertexStateDescriptor::vertex_buffers array
- create a vertex buffer with Device::create_buffer_with_data
- call RenderPass::set_vertex_buffer to set it

## index buffer
- create an index buffer with Device::create_buffer_with_data
- call RenderPass::set_index_buffer to set it
- call RenderPass::draw_indexed draw using the index buffer

## bind group
- create a BindGroupLayoutDescriptor to set up the structure of your bind group
    - add a number of BindGroupLayoutEntry
- create a BindGroupDescriptor for each bind group, this is like an instance of your bind group
    - add a number of Binding as declared in the layout
- call RenderPass::set_bind_group to set the BindGroupDescriptor

## render pipeline
- create PipelineLayoutDescriptor with Device::create_pipeline_layout

ORDER OF CALLING RenderPass::set_* statements (and draw) is important, do it in this order:
- set_pipeline
- set_bind_group
- set_vertex_buffer
- set_index_buffer
- draw_indexed


# App stack
- Executor      (Concurrency)
- Data          (State) 
- Event         (Changes to State, arguably part of State)
- Display       (Graphics)
- Serializer    (Persistent State and external communications)

GUI Applications need to: 
- Operate concurrently      (Threads, Execution model, async/await, Dispatch)
- Mutate & Read state       (Data model, ECS, Buffers, Structs)
- Persist state             (Serialization, Parsing, Files, Databases)
- Present info              (Graphics, Text, Sounds, Animations, Videos)
- Accept info               (Text, Microphone, Video, Images)
