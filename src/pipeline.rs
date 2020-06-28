use crate::camera::Camera;
use crate::instance::{Instance, InstanceRaw};
use crate::texture::Texture;
use crate::uniform::Uniform;
use crate::vertex::Vertex;

use cgmath::prelude::*;

// square quad
#[rustfmt::skip]
const VERTICES: &[Vertex] = &[

    Vertex { position: [-0.5,  0.5, 0.0], tex_coords: [0.4131759, 0.00759614], }, // top left
    Vertex { position: [ 0.5,  0.5, 0.0], tex_coords: [0.0048659444, 0.43041354], }, // top right
    Vertex { position: [ 0.5, -0.5, 0.0], tex_coords: [0.28081453, 0.949397057], }, // bottm right
    Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.85967, 0.84732911], }, // bottom left
];
#[rustfmt::skip]
const INDICES: &[u16] = &[
    0, 2, 1,
    0, 3, 2,
];
// pentagon
// #[rustfmt::skip]
// const VERTICES: &[Vertex] = &[
//     Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], }, // A
//     Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], }, // B
//     Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397057], }, // C
//     Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732911], }, // D
//     Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], }, // E
// ];
// #[rustfmt::skip]
// const INDICES: &[u16] = &[
//     0, 1, 4,
//     1, 2, 4,
//     2, 3, 4,
// ];

fn instances() -> Vec<Instance> {
    (0..5)
        .flat_map(|x| {
            (0..5).map(move |y| {
                let position = cgmath::Vector3 {
                    x: x as f32,
                    y: y as f32,
                    z: 0.0,
                };
                let rotation = cgmath::Quaternion::one();
                Instance { position, rotation }
            })
        })
        .collect()
}

pub(crate) struct Pipeline {
    device: wgpu::Device,
    queue: wgpu::Queue,
    swap_chain_descriptor: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    diffuse_bind_group: wgpu::BindGroup,
    uniform_bind_group: wgpu::BindGroup,
    instance_bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl Pipeline {
    pub fn new(
        surface: &wgpu::Surface,
        device: wgpu::Device,
        queue: wgpu::Queue,
        width: u32,
        height: u32,
    ) -> Pipeline {
        let (swap_chain_descriptor, swap_chain) =
            create_swap_chain(&surface, &device, width, height);

        let (diffuse_bind_group_layout, diffuse_bind_group) =
            create_diffuse_bind_group(&device, &queue);

        let (uniform_bind_group_layout, uniform_bind_group) =
            create_uniform_bind_group(&device, width, height);

        let (instance_bind_group_layout, instance_bind_group) = create_instance_bind_group(&device);

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[
                &diffuse_bind_group_layout,
                &uniform_bind_group_layout,
                &instance_bind_group_layout,
            ],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &create_vertex_shader(&device),
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &create_fragment_shader(&device),
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

        let vertex_buffer = device
            .create_buffer_with_data(bytemuck::cast_slice(VERTICES), wgpu::BufferUsage::VERTEX);

        let index_buffer =
            device.create_buffer_with_data(bytemuck::cast_slice(INDICES), wgpu::BufferUsage::INDEX);

        let num_indices = INDICES.len() as u32;

        Pipeline {
            device,
            queue,
            swap_chain_descriptor,
            swap_chain,
            diffuse_bind_group,
            uniform_bind_group,
            instance_bind_group,
            render_pipeline,
            index_buffer,
            vertex_buffer,
            num_indices,
        }
    }

    pub fn render_next_frame(&mut self, surface: &wgpu::Surface) {
        let frame = match self.swap_chain.get_next_frame() {
            Ok(frame) => frame,
            Err(_) => {
                self.swap_chain = self
                    .device
                    .create_swap_chain(&surface, &self.swap_chain_descriptor);
                self.swap_chain
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

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);
            render_pass.set_bind_group(2, &self.instance_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..));
            render_pass.draw_indexed(0..self.num_indices, 0, 0..25);
        }
        self.queue.submit(vec![encoder.finish()]);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.swap_chain_descriptor.width = width;
        self.swap_chain_descriptor.height = height;
    }
}

fn create_swap_chain(
    surface: &wgpu::Surface,
    device: &wgpu::Device,
    width: u32,
    height: u32,
) -> (wgpu::SwapChainDescriptor, wgpu::SwapChain) {
    let swap_chain_descriptor = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width,
        height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);
    (swap_chain_descriptor, swap_chain)
}

fn create_diffuse_bind_group(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
    let diffuse_bytes = include_bytes!("happy-tree.png");
    let diffuse_texture =
        Texture::from_bytes(&device, &queue, diffuse_bytes, "happy-tree.png").unwrap();

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

    let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &diffuse_bind_group_layout,
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
    (diffuse_bind_group_layout, diffuse_bind_group)
}

fn create_uniform_bind_group(
    device: &wgpu::Device,
    width: u32,
    height: u32,
) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
    let camera = Camera {
        eye: (0.0, 0.0, 1.0).into(),
        target: (0.0, 0.0, 0.0).into(),
        up: cgmath::Vector3::unit_y(),
        projection: crate::camera::Projection::Orthographic {
            left: -0.5,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: 0.0,
            far: 2.0,
        },
    };

    let mut uniforms = Uniform::new();
    uniforms.update_view_proj(&camera);
    let uniform_buffer = device.create_buffer_with_data(
        bytemuck::cast_slice(&[uniforms]),
        wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
    );

    let uniform_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutEntry::new(
                0,
                wgpu::ShaderStage::VERTEX,
                wgpu::BindingType::UniformBuffer {
                    dynamic: false,
                    min_binding_size: wgpu::BufferSize::new(std::mem::size_of_val(&uniforms) as _),
                },
            )],
            label: Some("uniform_bind_group_layout"),
        });
    let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &uniform_bind_group_layout,
        bindings: &[wgpu::Binding {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(uniform_buffer.slice(..)),
        }],
        label: Some("uniform_bind_group"),
    });
    (uniform_bind_group_layout, uniform_bind_group)
}

fn create_instance_bind_group(device: &wgpu::Device) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
    let instance_data = instances()
        .iter()
        .map(Instance::to_raw)
        .collect::<Vec<InstanceRaw>>();
    // we'll need the size for later
    let instance_buffer_size = instance_data.len() * std::mem::size_of::<cgmath::Matrix4<f32>>();
    let instance_buffer = device.create_buffer_with_data(
        bytemuck::cast_slice(&instance_data),
        wgpu::BufferUsage::STORAGE,
    );

    let instance_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[wgpu::BindGroupLayoutEntry::new(
                0,
                wgpu::ShaderStage::VERTEX,
                wgpu::BindingType::StorageBuffer {
                    dynamic: false,
                    min_binding_size: wgpu::BufferSize::new(instance_buffer_size as _),
                    readonly: false,
                },
            )],
            label: Some("instance_bind_group_layout"),
        });

    let instance_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &instance_bind_group_layout,
        bindings: &[wgpu::Binding {
            binding: 0,
            resource: wgpu::BindingResource::Buffer(
                instance_buffer.slice(0..(instance_buffer_size as u64)),
            ),
        }],
        label: Some("instance_bind_group"),
    });

    (instance_bind_group_layout, instance_bind_group)
}

fn create_vertex_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::include_spirv!("shader/shader.vert.spv"))
}

fn create_fragment_shader(device: &wgpu::Device) -> wgpu::ShaderModule {
    device.create_shader_module(wgpu::include_spirv!("shader/shader.frag.spv"))
}
