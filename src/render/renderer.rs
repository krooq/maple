use super::camera::Projection;
use super::instance::{Instance, InstanceRaw};
use super::texture::Texture;
use super::uniform::Uniform;
use super::vertex::Vertex;
use crate::render::camera::Camera;

pub(crate) struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    layouts: Layouts,
    bindings: Bindings,
    pipeline: Pipeline,
    buffers: Buffers,
}
struct Layouts {
    uniform_bind_group_layout: wgpu::BindGroupLayout,
    diffuse_bind_group_layout: wgpu::BindGroupLayout,
    instance_bind_group_layout: wgpu::BindGroupLayout,
    pipeline_layout: wgpu::PipelineLayout,
}

struct Bindings {
    diffuse_bind_group: wgpu::BindGroup,
    uniform_bind_group: wgpu::BindGroup,
    instance_bind_group: wgpu::BindGroup,
}
struct Pipeline {
    swap_chain_descriptor: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    render_pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub fn new(
        surface: &wgpu::Surface,
        device: wgpu::Device,
        queue: wgpu::Queue,
        width: u32,
        height: u32,
    ) -> Self {
        let layouts = Layouts::new(&device);
        let bindings = Bindings::new(&device, &queue, &layouts);
        let pipeline = Pipeline::new(&device, surface, &layouts, width, height);
        let buffers = Buffers::new(&device);
        Self {
            device,
            queue,
            layouts,
            bindings,
            pipeline,
            buffers,
        }
    }

    pub fn draw_frame(
        &mut self,
        surface: &wgpu::Surface,
        vertices: &[Vertex],
        indices: &[u16],
        instances: &[Instance],
    ) {
        let instance_data = instances
            .iter()
            .map(Instance::to_raw)
            .collect::<Vec<InstanceRaw>>();
        // we'll need the size for later
        let instance_buffer_size =
            instance_data.len() * std::mem::size_of::<cgmath::Matrix4<f32>>();
        let instance_buffer = self.device.create_buffer_with_data(
            bytemuck::cast_slice(&instance_data),
            wgpu::BufferUsage::STORAGE,
        );
        let instance_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.layouts.instance_bind_group_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    instance_buffer.slice(0..(instance_buffer_size as u64)),
                ),
            }],
            label: Some("instance_bind_group"),
        });

        self.bindings.instance_bind_group = instance_bind_group;

        self.buffers.vertex_buffer = self
            .device
            .create_buffer_with_data(bytemuck::cast_slice(vertices), wgpu::BufferUsage::VERTEX);

        self.buffers.index_buffer = self
            .device
            .create_buffer_with_data(bytemuck::cast_slice(indices), wgpu::BufferUsage::INDEX);

        self.buffers.num_indices = indices.len() as u32;

        let frame = match self.pipeline.swap_chain.get_next_frame() {
            Ok(frame) => frame,
            Err(_) => {
                self.pipeline.swap_chain = self
                    .device
                    .create_swap_chain(&surface, &self.pipeline.swap_chain_descriptor);
                self.pipeline
                    .swap_chain
                    .get_next_frame()
                    .expect("Failed to acquire next swap chain texture!")
            }
        };

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.output.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: false,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.pipeline.render_pipeline);
            render_pass.set_bind_group(0, &self.bindings.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.bindings.uniform_bind_group, &[]);
            render_pass.set_bind_group(2, &self.bindings.instance_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.buffers.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.buffers.index_buffer.slice(..));
            render_pass.draw_indexed(0..self.buffers.num_indices, 0, 0..instances.len() as u32);
        }
        self.queue.submit(Some(encoder.finish()));
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.pipeline.swap_chain_descriptor.width = width;
        self.pipeline.swap_chain_descriptor.height = height;
    }
}

impl Layouts {
    fn new(device: &wgpu::Device) -> Self {
        let diffuse_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[
                    wgpu::BindGroupLayoutEntry::new(
                        0,
                        wgpu::ShaderStage::FRAGMENT,
                        wgpu::BindingType::SampledTexture {
                            multisampled: false,
                            dimension: wgpu::TextureViewDimension::D2,
                            component_type: wgpu::TextureComponentType::Uint,
                        },
                    ),
                    wgpu::BindGroupLayoutEntry::new(
                        1,
                        wgpu::ShaderStage::FRAGMENT,
                        wgpu::BindingType::Sampler { comparison: false },
                    ),
                ],
                label: Some("diffuse_bind_group_layout"),
            });
        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[wgpu::BindGroupLayoutEntry::new(
                    0,
                    wgpu::ShaderStage::VERTEX,
                    wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: None,
                    },
                )],
                label: Some("uniform_bind_group_layout"),
            });
        let instance_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                bindings: &[wgpu::BindGroupLayoutEntry::new(
                    0,
                    wgpu::ShaderStage::VERTEX,
                    wgpu::BindingType::StorageBuffer {
                        dynamic: false,
                        min_binding_size: None,
                        readonly: false,
                    },
                )],
                label: Some("instance_bind_group_layout"),
            });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[
                &diffuse_bind_group_layout,
                &uniform_bind_group_layout,
                &instance_bind_group_layout,
            ],
        });
        Self {
            uniform_bind_group_layout,
            diffuse_bind_group_layout,
            instance_bind_group_layout,
            pipeline_layout,
        }
    }
}
impl Bindings {
    fn new(device: &wgpu::Device, queue: &wgpu::Queue, layouts: &Layouts) -> Self {
        let diffuse_bytes = include_bytes!("../images/happy-tree.png");
        let diffuse_texture =
            Texture::from_bytes(&device, &queue, diffuse_bytes, "happy-tree.png").unwrap();

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layouts.diffuse_bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let camera = Camera {
            eye: (0.0, 0.0, 1.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: cgmath::Vector3::unit_y(),
            projection: Projection::Orthographic {
                left: -1.0,
                right: 1.0,
                bottom: -1.0,
                top: 1.0,
                near: -1.0,
                far: 2.0,
            },
        };
        let mut uniforms = Uniform::new();
        uniforms.update_view_proj(&camera);
        let uniform_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(&[uniforms]),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layouts.uniform_bind_group_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(uniform_buffer.slice(..)),
            }],
            label: Some("uniform_bind_group"),
        });

        let instance_data = Vec::new()
            .iter()
            .map(Instance::to_raw)
            .collect::<Vec<InstanceRaw>>();

        //TODO: dont create this empty instance buffer
        let instance_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(&instance_data),
            wgpu::BufferUsage::STORAGE,
        );
        let instance_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layouts.instance_bind_group_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(instance_buffer.slice(..)),
            }],
            label: Some("instance_bind_group"),
        });
        Bindings {
            diffuse_bind_group,
            uniform_bind_group,
            instance_bind_group,
        }
    }
}

impl Pipeline {
    fn new(
        device: &wgpu::Device,
        surface: &wgpu::Surface,
        layouts: &Layouts,
        width: u32,
        height: u32,
    ) -> Self {
        let swap_chain_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width,
            height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &layouts.pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &device
                    .create_shader_module(wgpu::include_spirv!("shader/shader.vert.spv")),
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &device
                    .create_shader_module(wgpu::include_spirv!("shader/shader.frag.spv")),
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                ..Default::default()
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: swap_chain_descriptor.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[Vertex::desc()],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Self {
            swap_chain_descriptor,
            swap_chain,
            render_pipeline,
        }
    }
}
struct Buffers {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}
impl Buffers {
    fn new(device: &wgpu::Device) -> Self {
        let vertices = &Vec::<Vertex>::new()[..];
        let indices = &Vec::<u16>::new()[..];

        let vertex_buffer = device
            .create_buffer_with_data(bytemuck::cast_slice(vertices), wgpu::BufferUsage::VERTEX);

        let index_buffer =
            device.create_buffer_with_data(bytemuck::cast_slice(indices), wgpu::BufferUsage::INDEX);

        let num_indices = indices.len() as u32;
        Self {
            vertex_buffer,
            index_buffer,
            num_indices,
        }
    }
}
