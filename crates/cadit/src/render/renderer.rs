use std::sync::Arc;

use eframe::epaint::PaintCallbackInfo;
use egui_winit_vulkano::RenderResources;
use vulkano::{
    buffer::{CpuAccessibleBuffer, TypedBufferAccess},
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, PrimaryCommandBufferAbstract,
        RenderPassBeginInfo, SubpassContents,
    },
    descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet},
    format::{ClearValue, Format},
    image::{
        view::ImageView, AttachmentImage, ImageDimensions, ImageUsage, ImageViewAbstract,
        SampleCount, StorageImage,
    },
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

use super::{
    cgmath_types::{Point3, Vec2, Vec3},
    model::{BufferedEdgeVertex, BufferedSurfaceVertex},
    scene::Scene,
};

const IMAGE_FORMAT: Format = Format::B8G8R8A8_UNORM;

pub struct Renderer {
    scene: Scene,
    render_pass: Arc<RenderPass>,
    surface_pipeline: Arc<GraphicsPipeline>,
    edge_pipeline: Arc<GraphicsPipeline>,
    images: RendererImages,
    msaa_samples: SampleCount,
    scissor: Scissor,
    framebuffers_rebuilt: bool,

    // Surface buffers
    surface_vertex_buffer: Arc<CpuAccessibleBuffer<[BufferedSurfaceVertex]>>,
    surface_index_buffer: Arc<CpuAccessibleBuffer<[u32]>>,
    surface_descriptor_set: Arc<PersistentDescriptorSet>,

    // Edge buffers
    edge_vertex_buffer: Arc<CpuAccessibleBuffer<[BufferedEdgeVertex]>>,
}
impl Renderer {
    pub fn new<'a>(
        scene: Scene,
        resources: &RenderResources<'a>,
        msaa_samples: SampleCount,
    ) -> Self {
        let scissor = Scissor {
            origin: [0, 0],
            dimensions: [0, 0],
        };

        let (render_pass, images, surface_pipeline, edge_pipeline) =
            Self::create_pipelines(resources, msaa_samples, &scissor);

        let (surface_vertex_buffer, surface_index_buffer) =
            scene.surface_geometry_buffers(&resources.memory_allocator);

        let edge_vertex_buffer = scene.edge_geometry_buffer(&resources.memory_allocator);

        let (ambient_light_buffer, directional_light_buffer, point_light_buffer) =
            scene.lights().light_buffers(&resources.memory_allocator);

        let material_buffer = scene.material_buffer(&resources.memory_allocator);

        let surface_descriptor_set = PersistentDescriptorSet::new(
            resources.descriptor_set_allocator,
            surface_pipeline
                .layout()
                .set_layouts()
                .get(0)
                .unwrap()
                .clone(),
            [
                WriteDescriptorSet::buffer(0, point_light_buffer),
                WriteDescriptorSet::buffer(1, ambient_light_buffer),
                WriteDescriptorSet::buffer(2, directional_light_buffer),
                WriteDescriptorSet::buffer(3, material_buffer),
            ],
        )
        .unwrap();

        Self {
            scene,
            render_pass,
            surface_pipeline,
            edge_pipeline,
            images,
            msaa_samples,
            scissor,
            framebuffers_rebuilt: true,

            // Surface buffers
            surface_vertex_buffer,
            surface_index_buffer,
            surface_descriptor_set,

            // Edge buffers
            edge_vertex_buffer,
        }
    }

    pub fn camera_vec_to(&self, location: Point3) -> Vec3 {
        self.scene.camera().vec_to(location)
    }

    pub fn viewport_size_at_dist(&self, dist: f32) -> Vec2 {
        self.scene.camera().viewport_size_at_dist(dist)
    }

    pub fn framebuffers_rebuilt(&self) -> bool {
        self.framebuffers_rebuilt
    }

    fn create_pipelines<'a>(
        resources: &RenderResources<'a>,
        msaa_samples: SampleCount,
        scissor: &Scissor,
    ) -> (
        Arc<RenderPass>,
        RendererImages,
        Arc<GraphicsPipeline>,
        Arc<GraphicsPipeline>,
    ) {
        let device = resources.queue.device();
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
                        }
                    ]
                )
                .unwrap()
            }
        };

        let images = RendererImages::new(
            render_pass.clone(),
            resources,
            &scissor,
            msaa_samples,
            IMAGE_FORMAT,
        );

        let surface_pipeline = GraphicsPipeline::start()
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
                surface_fs::load(device.clone())
                    .unwrap()
                    .entry_point("main")
                    .unwrap(),
                (),
            )
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
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
            .input_assembly_state(InputAssemblyState::new().topology(PrimitiveTopology::LineList))
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
            .render_pass(Subpass::from(render_pass.clone(), 1).unwrap())
            .build(device.clone())
            .unwrap();

        (render_pass, images, surface_pipeline, edge_pipeline)
    }

    fn update_viewport<'a>(&mut self, info: &PaintCallbackInfo, resources: &RenderResources<'a>) {
        let vpip = info.viewport_in_pixels();
        let new_scissor = Scissor {
            origin: [vpip.left_px as u32, vpip.top_px as u32],
            dimensions: [vpip.width_px as u32, vpip.height_px as u32],
        };

        if new_scissor != self.scissor {
            self.scissor = new_scissor;
            self.scene
                .camera_mut()
                .set_viewport_in_pixels(self.scissor.dimensions);
            self.images = RendererImages::new(
                self.render_pass.clone(),
                resources,
                &self.scissor,
                self.msaa_samples,
                IMAGE_FORMAT,
            );

            self.framebuffers_rebuilt = true;
        } else {
            self.framebuffers_rebuilt = false;
        }
    }

    pub fn render<'a>(&mut self, info: &PaintCallbackInfo, resources: &RenderResources<'a>) {
        self.update_viewport(info, resources);

        let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
            resources.command_buffer_allocator,
            resources.queue.queue_family_index(),
            CommandBufferUsage::MultipleSubmit,
        )
        .unwrap();

        let push_constants = surface_vs::ty::PushConstants {
            model_matrix: self.scene.orientation().matrix().into(),
            projection_matrix: self.scene.camera().projection_matrix().into(),
        };

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
            )
            // Surface commands
            .bind_pipeline_graphics(self.surface_pipeline.clone())
            .bind_vertex_buffers(0, self.surface_vertex_buffer.clone())
            .bind_index_buffer(self.surface_index_buffer.clone())
            .bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                self.surface_pipeline.layout().clone(),
                0,
                self.surface_descriptor_set.clone(),
            )
            .push_constants(self.surface_pipeline.layout().clone(), 0, push_constants)
            .draw_indexed(self.surface_index_buffer.len() as u32, 1, 0, 0, 0)
            .unwrap()
            // Edge commands
            .next_subpass(SubpassContents::Inline)
            .unwrap()
            .bind_pipeline_graphics(self.edge_pipeline.clone())
            .bind_vertex_buffers(0, self.edge_vertex_buffer.clone())
            .draw(self.edge_vertex_buffer.len() as u32, 1, 0, 0)
            .unwrap()
            // Finish
            .end_render_pass()
            .unwrap();

        let command_buffer = command_buffer_builder.build().unwrap();

        let finished = command_buffer.execute(resources.queue.clone()).unwrap();
        finished
            .then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .unwrap();
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
        resources: &RenderResources,
        scissor: &Scissor,
        samples: SampleCount,
        format: Format,
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
                AttachmentImage::multisampled(
                    &resources.memory_allocator,
                    dimensions,
                    samples,
                    format,
                )
                .unwrap(),
            )
            .unwrap();

            attachments.push(intermediary);
        }

        let depth = ImageView::new_default(
            AttachmentImage::multisampled_with_usage(
                &resources.memory_allocator,
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
            &resources.memory_allocator,
            ImageDimensions::Dim2d {
                width: dimensions[0],
                height: dimensions[1],
                array_layers: 1,
            },
            format,
            Some(resources.queue.queue_family_index()),
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
        path: "src/render/shaders/surface.vert",
        types_meta: {
            #[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
        },
    }
}

mod surface_fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/render/shaders/surface.frag",
    }
}

mod edge_vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/render/shaders/edge.vert",
    }
}

mod edge_fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/render/shaders/edge.frag",
    }
}
