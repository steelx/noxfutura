use super::{Camera, Uniforms, gbuffer};
use crate::engine::{Context, VertexBuffer};

pub struct BlockRenderPass {
    pub vb: VertexBuffer<f32>,
    pub camera: Camera,
    pub uniforms: Uniforms,
    pub shader_id: usize,
    pub uniform_bind_group: wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
    pub uniform_buf: wgpu::Buffer,
    pub terrain_textures: super::texarray::TextureArray,
    terrain_bind_group: wgpu::BindGroup,
    gbuffer : gbuffer::GBuffer
}

impl BlockRenderPass {
    pub fn new(context: &mut Context) -> Self {
        let terrain_textures = super::texarray::TextureArray::blank(context).unwrap();

        // Initialize the vertex buffer for cube geometry
        let mut vb = VertexBuffer::<f32>::new(&[3, 3, 2, 1]);
        let mut tmp = 0;
        crate::utils::add_cube_geometry(&mut vb.data, &mut tmp, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0);
        vb.build(&context.device, wgpu::BufferUsage::VERTEX);

        // Initialize camera and uniforms
        let camera = Camera::new(context.size.width, context.size.height);
        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&camera, 0);

        // Shader
        let shader_id = context.register_shader(
            "resources/shaders/regionblocks.vert",
            "resources/shaders/regionblocks.frag",
        );

        // WGPU Details
        let uniform_buf = context.device.create_buffer_with_data(
            bytemuck::cast_slice(&[uniforms]),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );
        let uniform_bind_group_layout =
            context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    bindings: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::VERTEX,
                        ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                    }],
                    label: None,
                });
        let uniform_bind_group = context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &uniform_bind_group_layout,
                bindings: &[wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buf,
                        range: 0..std::mem::size_of::<Uniforms>() as wgpu::BufferAddress,
                    },
                }],
                label: None,
            });

        // Terrain textures
        let terrain_bind_group_layout =
            context
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    bindings: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::SampledTexture {
                                multisampled: false,
                                dimension: wgpu::TextureViewDimension::D2,
                                component_type: wgpu::TextureComponentType::Uint,
                            },
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::Sampler { comparison: false },
                        },
                    ],
                    label: Some("texture_bind_group_layout"),
                });

        let terrain_bind_group = context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &terrain_bind_group_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&terrain_textures.view),
                    },
                    wgpu::Binding {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&terrain_textures.sampler),
                    },
                ],
                label: Some("diffuse_bind_group"),
            });

        let pipeline_layout =
            context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[&uniform_bind_group_layout, &terrain_bind_group_layout],
                });
        let render_pipeline =
            context
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    layout: &pipeline_layout,
                    vertex_stage: wgpu::ProgrammableStageDescriptor {
                        module: &context.shaders[shader_id].vs_module,
                        entry_point: "main",
                    },
                    fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                        module: &context.shaders[shader_id].fs_module,
                        entry_point: "main",
                    }),
                    rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: wgpu::CullMode::Back,
                        depth_bias: 0,
                        depth_bias_slope_scale: 0.0,
                        depth_bias_clamp: 0.0,
                    }),
                    primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                    color_states: &vec![
                        wgpu::ColorStateDescriptor {
                            format: context.swapchain_format,
                            color_blend: wgpu::BlendDescriptor::REPLACE,
                            alpha_blend: wgpu::BlendDescriptor::REPLACE,
                            write_mask: wgpu::ColorWrite::ALL,
                        },
                        wgpu::ColorStateDescriptor {
                            format: context.swapchain_format,
                            color_blend: wgpu::BlendDescriptor::REPLACE,
                            alpha_blend: wgpu::BlendDescriptor::REPLACE,
                            write_mask: wgpu::ColorWrite::ALL,
                        }
                    ],
                    depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                        format: crate::engine::texture::Texture::DEPTH_FORMAT,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
                        stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
                        stencil_read_mask: 0,
                        stencil_write_mask: 0,
                    }),
                    vertex_state: wgpu::VertexStateDescriptor {
                        index_format: wgpu::IndexFormat::Uint16,
                        vertex_buffers: &[vb.descriptor()],
                    },
                    sample_count: 1,
                    sample_mask: !0,
                    alpha_to_coverage_enabled: false,
                });

        // Build the result
        let builder = Self {
            vb,
            camera,
            uniforms,
            shader_id,
            uniform_bind_group,
            render_pipeline,
            uniform_buf,
            terrain_textures,
            terrain_bind_group,
            gbuffer : gbuffer::GBuffer::new(context)
        };
        builder
    }

    pub fn render(
        &mut self,
        context: &mut Context,
        depth_id: usize,
        frame: &wgpu::SwapChainOutput,
        chunks: &super::chunks::Chunks,
        camera_z: usize,
    ) {
        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &self.gbuffer.albedo.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color::BLUE,
                    },
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &self.gbuffer.normal.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color::RED,
                    }
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &context.textures[depth_id].view,
                    depth_load_op: wgpu::LoadOp::Clear,
                    depth_store_op: wgpu::StoreOp::Store,
                    clear_depth: 1.0,
                    stencil_load_op: wgpu::LoadOp::Clear,
                    stencil_store_op: wgpu::StoreOp::Store,
                    clear_stencil: 0,
                }),
            });

            rpass.set_pipeline(&self.render_pipeline);
            rpass.set_bind_group(0, &self.uniform_bind_group, &[]);
            rpass.set_bind_group(1, &self.terrain_bind_group, &[]);

            if self.vb.len() > 0 {
                rpass.set_vertex_buffer(0, &self.vb.buffer.as_ref().unwrap(), 0, 0);
                rpass.draw(0..self.vb.len(), 0..1);
            }

            for chunk in chunks.visible_chunks() {
                let buffer = chunk.maybe_render_chunk(camera_z);
                if let Some(buffer) = buffer {
                    rpass.set_vertex_buffer(0, buffer.0.buffer.as_ref().unwrap(), 0, 0);
                    rpass.draw(0..buffer.1, 0..1);
                }
            }
        }
        context.queue.submit(&[encoder.finish()]);
    }

    pub fn on_resize(&mut self, context: &mut crate::engine::Context) {
        self.gbuffer = gbuffer::GBuffer::new(context);
    }
}