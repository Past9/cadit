use std::sync::Arc;

use cgmath::{Point3, Vector2, Vector3};
use vulkano::{
    buffer::{CpuAccessibleBuffer, TypedBufferAccess},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        PrimaryAutoCommandBuffer, PrimaryCommandBufferAbstract, RenderPassBeginInfo,
        SubpassContents,
    },
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator, PersistentDescriptorSet, WriteDescriptorSet,
    },
    device::Queue,
    format::{ClearValue, Format},
    image::{
        view::ImageView, AttachmentImage, ImageDimensions, ImageUsage, ImageViewAbstract,
        SampleCount, StorageImage,
    },
    memory::allocator::StandardMemoryAllocator,
    pipeline::{
        graphics::{
            depth_stencil::{CompareOp, DepthState, DepthStencilState},
            input_assembly::{InputAssemblyState, PrimitiveTopology},
            multisample::MultisampleState,
            rasterization::{CullMode, FrontFace, LineRasterizationMode, RasterizationState},
            vertex_input::BuffersDefinition,
            viewport::{Scissor, Viewport, ViewportState},
        },
        GraphicsPipeline, Pipeline, PipelineBindPoint, StateMode,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    sync::GpuFuture,
};

use crate::PixelViewport;

use super::{
    model::{BufferedEdgeVertex, BufferedPointVertex, BufferedSurfaceVertex},
    scene::Scene,
};

const IMAGE_FORMAT: Format = Format::B8G8R8A8_UNORM;

pub struct Renderer {
    scene: Scene,
    render_pass: Arc<RenderPass>,
    opaque_surface_pipeline: Arc<GraphicsPipeline>,
    translucent_surface_pipeline: Arc<GraphicsPipeline>,
    edge_pipeline: Arc<GraphicsPipeline>,
    point_pipeline: Arc<GraphicsPipeline>,
    images: RendererImages,
    msaa_samples: SampleCount,
    scissor: Scissor,
    framebuffers_rebuilt: bool,

    // Opaque surface buffers
    opaque_surface_vertex_buffer: Option<Arc<CpuAccessibleBuffer<[BufferedSurfaceVertex]>>>,
    opaque_surface_index_buffer: Option<Arc<CpuAccessibleBuffer<[u32]>>>,
    opaque_surface_descriptor_set: Arc<PersistentDescriptorSet>,

    // Translucent surface buffers
    translucent_surface_vertex_buffer: Option<Arc<CpuAccessibleBuffer<[BufferedSurfaceVertex]>>>,
    translucent_surface_index_buffer: Option<Arc<CpuAccessibleBuffer<[u32]>>>,
    translucent_surface_descriptor_set: Arc<PersistentDescriptorSet>,

    // Edge buffers
    edge_vertex_buffer: Option<Arc<CpuAccessibleBuffer<[BufferedEdgeVertex]>>>,
    edge_index_buffer: Option<Arc<CpuAccessibleBuffer<[u32]>>>,

    // Point buffers
    point_vertex_buffer: Option<Arc<CpuAccessibleBuffer<[BufferedPointVertex]>>>,
}
impl Renderer {
    pub fn new<'a>(
        scene: Scene,
        msaa_samples: SampleCount,
        memory_allocator: &StandardMemoryAllocator,
        descriptor_set_allocator: &StandardDescriptorSetAllocator,
        queue: Arc<Queue>,
    ) -> Self {
        let scissor = Scissor {
            origin: [0, 0],
            dimensions: [0, 0],
        };

        let (
            render_pass,
            images,
            opaque_surface_pipeline,
            translucent_surface_pipeline,
            edge_pipeline,
            point_pipeline,
        ) = Self::create_pipelines(msaa_samples, &scissor, memory_allocator, queue);

        let (opaque_surface_vertex_buffer, opaque_surface_index_buffer) =
            scene.opaque_surface_geometry_buffers(memory_allocator);

        let (translucent_surface_vertex_buffer, translucent_surface_index_buffer) =
            scene.translucent_surface_geometry_buffers(memory_allocator);

        let (edge_vertex_buffer, edge_index_buffer) = scene.edge_geometry_buffers(memory_allocator);

        let point_vertex_buffer = scene.point_geometry_buffer(memory_allocator);

        let (ambient_light_buffer, directional_light_buffer, point_light_buffer) =
            scene.lights().light_buffers(memory_allocator);

        let (opaque_material_buffer, translucent_material_buffer) =
            scene.material_buffers(memory_allocator);

        let opaque_surface_descriptor_set = PersistentDescriptorSet::new(
            descriptor_set_allocator,
            opaque_surface_pipeline
                .layout()
                .set_layouts()
                .get(0)
                .unwrap()
                .clone(),
            [
                WriteDescriptorSet::buffer(0, point_light_buffer.clone()),
                WriteDescriptorSet::buffer(1, ambient_light_buffer.clone()),
                WriteDescriptorSet::buffer(2, directional_light_buffer.clone()),
                WriteDescriptorSet::buffer(3, opaque_material_buffer),
            ],
        )
        .unwrap();

        let translucent_surface_descriptor_set = PersistentDescriptorSet::new(
            descriptor_set_allocator,
            translucent_surface_pipeline
                .layout()
                .set_layouts()
                .get(0)
                .unwrap()
                .clone(),
            [
                WriteDescriptorSet::buffer(0, point_light_buffer),
                WriteDescriptorSet::buffer(1, ambient_light_buffer),
                WriteDescriptorSet::buffer(2, directional_light_buffer),
                WriteDescriptorSet::buffer(3, translucent_material_buffer),
            ],
        )
        .unwrap();

        Self {
            scene,
            render_pass,
            opaque_surface_pipeline,
            translucent_surface_pipeline,
            edge_pipeline,
            point_pipeline,
            images,
            msaa_samples,
            scissor,
            framebuffers_rebuilt: true,

            // Opaque surface buffers
            opaque_surface_vertex_buffer,
            opaque_surface_index_buffer,
            opaque_surface_descriptor_set,

            // Translucent surface buffers
            translucent_surface_vertex_buffer,
            translucent_surface_index_buffer,
            translucent_surface_descriptor_set,

            // Edge buffers
            edge_vertex_buffer,
            edge_index_buffer,

            // Point buffers
            point_vertex_buffer,
        }
    }

    pub fn camera_vec_to(&self, location: Point3<f32>) -> Vector3<f32> {
        self.scene.camera().vec_to(location)
    }

    pub fn viewport_size_at_dist(&self, dist: f32) -> Vector2<f32> {
        self.scene.camera().viewport_size_at_dist(dist)
    }

    pub fn framebuffers_rebuilt(&self) -> bool {
        self.framebuffers_rebuilt
    }

    fn create_pipelines<'a>(
        msaa_samples: SampleCount,
        scissor: &Scissor,
        memory_allocator: &StandardMemoryAllocator,
        queue: Arc<Queue>,
    ) -> (
        Arc<RenderPass>,
        RendererImages,
        Arc<GraphicsPipeline>,
        Arc<GraphicsPipeline>,
        Arc<GraphicsPipeline>,
        Arc<GraphicsPipeline>,
    ) {
        let device = queue.device();
        let render_pass = {
            if msaa_samples == SampleCount::Sample1 {
                vulkano::single_pass_renderpass!(
                    device.clone(),
                    attachments: {
                        depth: {
                            load: Clear,
                            store: DontCare,
                            format: Format::D32_SFLOAT,
                            samples: 1,
                        },
                        color: {
                            load: Clear,
                            store: Store,
                            format: IMAGE_FORMAT,
                            samples: 1,
                        }
                    },
                    pass: {
                        color: [color],
                        depth_stencil: {depth}
                    }
                )
                .unwrap()
            } else {
                vulkano::ordered_passes_renderpass!(
                    device.clone(),
                    attachments: {
                        intermediary: {
                            load: Clear,
                            store: DontCare,
                            format: IMAGE_FORMAT,
                            samples: msaa_samples,
                        },
                        depth: {
                            load: Clear,
                            store: DontCare,
                            format: Format::D32_SFLOAT,
                            samples: msaa_samples,
                        },
                        color: {
                            load: DontCare,
                            store: Store,
                            format: IMAGE_FORMAT,
                            samples: 1,
                        }
                    },
                    passes: [
                        {
                            color: [intermediary],
                            depth_stencil: {depth},
                            input: [],
                            resolve: [color]
                        },
                        {
                            color: [intermediary],
                            depth_stencil: {depth},
                            input: [],
                            resolve: [color]
                        },
                        {
                            color: [intermediary],
                            depth_stencil: {depth},
                            input: [],
                            resolve: [color]
                        },
                        {
                            color: [intermediary],
                            depth_stencil: {depth},
                            input: [],
                            resolve: [color]
                        }
                    ]
                )
                .unwrap()
            }
        };

        let images = RendererImages::new(
            render_pass.clone(),
            &scissor,
            msaa_samples,
            IMAGE_FORMAT,
            memory_allocator,
            queue.clone(),
        );

        let opaque_surface_pipeline = GraphicsPipeline::start()
            .vertex_input_state(BuffersDefinition::new().vertex::<BufferedSurfaceVertex>())
            .vertex_shader(
                surface_vs::load(device.clone())
                    .unwrap()
                    .entry_point("main")
                    .unwrap(),
                (),
            )
            .input_assembly_state(
                InputAssemblyState::new().topology(PrimitiveTopology::TriangleList),
            )
            .rasterization_state(RasterizationState {
                front_face: StateMode::Fixed(FrontFace::CounterClockwise),
                cull_mode: StateMode::Fixed(CullMode::Back),
                ..RasterizationState::default()
            })
            .multisample_state(MultisampleState {
                rasterization_samples: msaa_samples,
                sample_shading: Some(0.5),
                ..Default::default()
            })
            .depth_stencil_state(DepthStencilState {
                depth: Some(DepthState {
                    enable_dynamic: false,
                    write_enable: StateMode::Fixed(true),
                    compare_op: StateMode::Fixed(CompareOp::Less),
                }),
                ..DepthStencilState::default()
            })
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .fragment_shader(
                opaque_surface_fs::load(device.clone())
                    .unwrap()
                    .entry_point("main")
                    .unwrap(),
                (),
            )
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap();

        let translucent_surface_pipeline = GraphicsPipeline::start()
            .vertex_input_state(BuffersDefinition::new().vertex::<BufferedSurfaceVertex>())
            .vertex_shader(
                surface_vs::load(device.clone())
                    .unwrap()
                    .entry_point("main")
                    .unwrap(),
                (),
            )
            .input_assembly_state(
                InputAssemblyState::new().topology(PrimitiveTopology::TriangleList),
            )
            .rasterization_state(RasterizationState {
                front_face: StateMode::Fixed(FrontFace::CounterClockwise),
                cull_mode: StateMode::Fixed(CullMode::Back),
                ..RasterizationState::default()
            })
            .multisample_state(MultisampleState {
                rasterization_samples: msaa_samples,
                sample_shading: Some(0.5),
                ..Default::default()
            })
            .depth_stencil_state(DepthStencilState {
                depth: Some(DepthState {
                    enable_dynamic: false,
                    write_enable: StateMode::Fixed(true),
                    compare_op: StateMode::Fixed(CompareOp::Always),
                }),
                ..DepthStencilState::default()
            })
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .fragment_shader(
                translucent_surface_fs::load(device.clone())
                    .unwrap()
                    .entry_point("main")
                    .unwrap(),
                (),
            )
            .render_pass(Subpass::from(render_pass.clone(), 1).unwrap())
            .build(device.clone())
            .unwrap();

        let edge_pipeline = GraphicsPipeline::start()
            .vertex_input_state(BuffersDefinition::new().vertex::<BufferedEdgeVertex>())
            .vertex_shader(
                edge_vs::load(device.clone())
                    .unwrap()
                    .entry_point("main")
                    .unwrap(),
                (),
            )
            .input_assembly_state(
                InputAssemblyState::new()
                    .topology(PrimitiveTopology::LineStrip)
                    .primitive_restart_enable(),
            )
            .rasterization_state(RasterizationState {
                front_face: StateMode::Fixed(FrontFace::CounterClockwise),
                cull_mode: StateMode::Fixed(CullMode::None),
                line_width: StateMode::Fixed(2.0),
                line_rasterization_mode: LineRasterizationMode::Rectangular,
                ..RasterizationState::default()
            })
            .multisample_state(MultisampleState {
                rasterization_samples: msaa_samples,
                sample_shading: Some(0.5),
                ..Default::default()
            })
            .depth_stencil_state(DepthStencilState {
                depth: Some(DepthState {
                    enable_dynamic: false,
                    write_enable: StateMode::Fixed(true),
                    compare_op: StateMode::Fixed(CompareOp::Less),
                }),
                ..DepthStencilState::default()
            })
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .fragment_shader(
                edge_fs::load(device.clone())
                    .unwrap()
                    .entry_point("main")
                    .unwrap(),
                (),
            )
            .render_pass(Subpass::from(render_pass.clone(), 2).unwrap())
            .build(device.clone())
            .unwrap();

        let point_pipeline = GraphicsPipeline::start()
            .vertex_input_state(BuffersDefinition::new().vertex::<BufferedPointVertex>())
            .vertex_shader(
                point_vs::load(device.clone())
                    .unwrap()
                    .entry_point("main")
                    .unwrap(),
                (),
            )
            .input_assembly_state(InputAssemblyState::new().topology(PrimitiveTopology::PointList))
            .rasterization_state(RasterizationState {
                front_face: StateMode::Fixed(FrontFace::CounterClockwise),
                cull_mode: StateMode::Fixed(CullMode::None),
                ..RasterizationState::default()
            })
            .multisample_state(MultisampleState {
                rasterization_samples: msaa_samples,
                sample_shading: Some(0.5),
                ..Default::default()
            })
            .depth_stencil_state(DepthStencilState {
                depth: Some(DepthState {
                    enable_dynamic: false,
                    write_enable: StateMode::Fixed(true),
                    compare_op: StateMode::Fixed(CompareOp::Less),
                }),
                ..DepthStencilState::default()
            })
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .fragment_shader(
                point_fs::load(device.clone())
                    .unwrap()
                    .entry_point("main")
                    .unwrap(),
                (),
            )
            .render_pass(Subpass::from(render_pass.clone(), 3).unwrap())
            .build(device.clone())
            .unwrap();

        (
            render_pass,
            images,
            opaque_surface_pipeline,
            translucent_surface_pipeline,
            edge_pipeline,
            point_pipeline,
        )
    }

    fn update_viewport<'a>(
        &mut self,
        pixel_viewport: &PixelViewport,
        memory_allocator: &StandardMemoryAllocator,
        queue: Arc<Queue>,
    ) {
        let new_scissor = Scissor {
            origin: [pixel_viewport.left, pixel_viewport.top],
            dimensions: [pixel_viewport.width, pixel_viewport.height],
        };

        if new_scissor != self.scissor {
            self.scissor = new_scissor;
            self.scene
                .camera_mut()
                .set_viewport_in_pixels(self.scissor.dimensions);
            self.images = RendererImages::new(
                self.render_pass.clone(),
                &self.scissor,
                self.msaa_samples,
                IMAGE_FORMAT,
                memory_allocator,
                queue,
            );

            self.framebuffers_rebuilt = true;
        } else {
            self.framebuffers_rebuilt = false;
        }
    }

    pub fn render<'a>(
        &mut self,
        pixel_viewport: &PixelViewport,
        memory_allocator: &StandardMemoryAllocator,
        command_buffer_allocator: &StandardCommandBufferAllocator,
        queue: Arc<Queue>,
    ) {
        self.update_viewport(pixel_viewport, memory_allocator, queue.clone());

        let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
            command_buffer_allocator,
            queue.queue_family_index(),
            CommandBufferUsage::MultipleSubmit,
        )
        .unwrap();

        let clear_values = {
            let bg_color = self.scene.bg_color().to_floats();

            let mut clear_values: Vec<Option<ClearValue>> = Vec::new();
            if self.msaa_samples != SampleCount::Sample1 {
                clear_values.push(Some(bg_color.into()));
            }

            clear_values.push(Some(1.0.into()));
            clear_values.push(Some(bg_color.into()));

            clear_values
        };

        command_buffer_builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: clear_values,
                    render_area_offset: self.scissor.origin,
                    render_area_extent: self.scissor.dimensions,
                    ..RenderPassBeginInfo::framebuffer(self.images.framebuffer.clone())
                },
                SubpassContents::Inline,
            )
            .unwrap()
            .set_viewport(
                0,
                [Viewport {
                    origin: [self.scissor.origin[0] as f32, self.scissor.origin[1] as f32],
                    dimensions: [
                        self.scissor.dimensions[0] as f32,
                        self.scissor.dimensions[1] as f32,
                    ],
                    depth_range: 0.0..1.0,
                }],
            );

        self.add_opaque_surface_commands(&mut command_buffer_builder);
        self.add_translucent_surface_commands(&mut command_buffer_builder);
        self.add_edge_commands(&mut command_buffer_builder);
        self.add_point_commands(&mut command_buffer_builder);

        command_buffer_builder.end_render_pass().unwrap();

        let command_buffer = command_buffer_builder.build().unwrap();

        let finished = command_buffer.execute(queue).unwrap();
        finished
            .then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .unwrap();
    }

    fn add_opaque_surface_commands(
        &self,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    ) {
        let push_constants = surface_vs::ty::PushConstants {
            model_matrix: self.scene.orientation().matrix().into(),
            projection_matrix: self.scene.camera().projection_matrix().into(),
        };

        builder
            .bind_pipeline_graphics(self.opaque_surface_pipeline.clone())
            .push_constants(
                self.opaque_surface_pipeline.layout().clone(),
                0,
                push_constants,
            );

        if let Some(ref surface_vertex_buffer) = self.opaque_surface_vertex_buffer {
            if let Some(ref surface_index_buffer) = self.opaque_surface_index_buffer {
                builder
                    .bind_vertex_buffers(0, surface_vertex_buffer.clone())
                    .bind_index_buffer(surface_index_buffer.clone())
                    .bind_descriptor_sets(
                        PipelineBindPoint::Graphics,
                        self.opaque_surface_pipeline.layout().clone(),
                        0,
                        self.opaque_surface_descriptor_set.clone(),
                    )
                    .draw_indexed(surface_index_buffer.len() as u32, 1, 0, 0, 0)
                    .unwrap();
            }
        }
    }

    fn add_translucent_surface_commands(
        &self,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    ) {
        let push_constants = surface_vs::ty::PushConstants {
            model_matrix: self.scene.orientation().matrix().into(),
            projection_matrix: self.scene.camera().projection_matrix().into(),
        };

        builder
            .next_subpass(SubpassContents::Inline)
            .unwrap()
            .bind_pipeline_graphics(self.translucent_surface_pipeline.clone())
            .push_constants(
                self.translucent_surface_pipeline.layout().clone(),
                0,
                push_constants,
            );

        if let Some(ref surface_vertex_buffer) = self.translucent_surface_vertex_buffer {
            if let Some(ref surface_index_buffer) = self.translucent_surface_index_buffer {
                builder
                    .bind_vertex_buffers(0, surface_vertex_buffer.clone())
                    .bind_index_buffer(surface_index_buffer.clone())
                    .bind_descriptor_sets(
                        PipelineBindPoint::Graphics,
                        self.translucent_surface_pipeline.layout().clone(),
                        0,
                        self.translucent_surface_descriptor_set.clone(),
                    )
                    .draw_indexed(surface_index_buffer.len() as u32, 1, 0, 0, 0)
                    .unwrap();
            }
        }
    }

    fn add_edge_commands(&self, builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>) {
        builder
            .next_subpass(SubpassContents::Inline)
            .unwrap()
            .bind_pipeline_graphics(self.edge_pipeline.clone());

        if let Some(ref edge_vertex_buffer) = self.edge_vertex_buffer {
            if let Some(ref edge_index_buffer) = self.edge_index_buffer {
                builder
                    .bind_vertex_buffers(0, edge_vertex_buffer.clone())
                    .bind_index_buffer(edge_index_buffer.clone())
                    .draw_indexed(edge_index_buffer.len() as u32, 1, 0, 0, 0)
                    .unwrap();
            }
        }
    }

    fn add_point_commands(&self, builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>) {
        builder
            .next_subpass(SubpassContents::Inline)
            .unwrap()
            .bind_pipeline_graphics(self.point_pipeline.clone());

        if let Some(ref point_vertex_buffer) = self.point_vertex_buffer {
            builder
                .bind_vertex_buffers(0, point_vertex_buffer.clone())
                .draw(point_vertex_buffer.len() as u32, 1, 0, 0)
                .unwrap();
        }
    }

    pub fn view(&self) -> Arc<dyn ImageViewAbstract> {
        self.images.view.clone()
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }
}

struct RendererImages {
    framebuffer: Arc<Framebuffer>,
    view: Arc<ImageView<StorageImage>>,
}
impl RendererImages {
    fn new(
        render_pass: Arc<RenderPass>,
        scissor: &Scissor,
        samples: SampleCount,
        format: Format,
        memory_allocator: &StandardMemoryAllocator,
        queue: Arc<Queue>,
    ) -> Self {
        // Make sure the images are at least 1 pixel in each dimension or Vulkan will
        // throw an error. Also make the images cover the offset area so they line up
        // with the egui area that we're painting. We'll only render to the area that
        // will be shown by egui.
        //
        // TODO: Is there a way to do this that doesn't waste memory on the offset area?
        let dimensions = [
            match scissor.dimensions[0] > 0 {
                true => scissor.dimensions[0],
                false => 1,
            } + scissor.origin[0],
            match scissor.dimensions[1] > 0 {
                true => scissor.dimensions[1],
                false => 1,
            } + scissor.origin[1],
        ];

        let mut attachments: Vec<Arc<dyn ImageViewAbstract>> = Vec::new();

        if samples != SampleCount::Sample1 {
            let intermediary = ImageView::new_default(
                AttachmentImage::multisampled(memory_allocator, dimensions, samples, format)
                    .unwrap(),
            )
            .unwrap();

            attachments.push(intermediary);
        }

        let depth = ImageView::new_default(
            AttachmentImage::multisampled_with_usage(
                memory_allocator,
                dimensions,
                samples,
                Format::D32_SFLOAT,
                ImageUsage {
                    depth_stencil_attachment: true,
                    transient_attachment: true,
                    ..ImageUsage::empty()
                },
            )
            .unwrap(),
        )
        .unwrap();

        attachments.push(depth);

        let color = StorageImage::new(
            memory_allocator,
            ImageDimensions::Dim2d {
                width: dimensions[0],
                height: dimensions[1],
                array_layers: 1,
            },
            format,
            Some(queue.queue_family_index()),
        )
        .unwrap();

        let view = ImageView::new_default(color.clone()).unwrap();

        attachments.push(view.clone());

        let framebuffer = Framebuffer::new(
            render_pass,
            FramebufferCreateInfo {
                attachments,
                ..Default::default()
            },
        )
        .unwrap();

        Self { framebuffer, view }
    }
}

mod surface_vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/shaders/surface.vert",
        types_meta: {
            #[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
        },
    }
}

mod opaque_surface_fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/shaders/opaque_surface.frag",
    }
}

mod translucent_surface_fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/shaders/translucent_surface.frag",
    }
}

mod edge_vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/shaders/edge.vert",
    }
}

mod edge_fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/shaders/edge.frag",
    }
}

mod point_vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/shaders/point.vert",
    }
}

mod point_fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/shaders/point.frag",
    }
}
