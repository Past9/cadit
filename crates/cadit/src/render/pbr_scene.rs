use std::sync::Arc;

use eframe::epaint::PaintCallbackInfo;
use egui_winit_vulkano::RenderResources;
use vulkano::{
    buffer::TypedBufferAccess,
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, PrimaryCommandBufferAbstract,
        RenderPassBeginInfo, SubpassContents,
    },
    format::Format,
    image::{
        view::ImageView, AttachmentImage, ImageDimensions, ImageViewAbstract, SampleCount,
        StorageImage,
    },
    pipeline::{
        graphics::{
            depth_stencil::{CompareOp, DepthState, DepthStencilState},
            input_assembly::{InputAssemblyState, PrimitiveTopology},
            multisample::MultisampleState,
            rasterization::{CullMode, FrontFace, RasterizationState},
            vertex_input::BuffersDefinition,
            viewport::ViewportState,
        },
        GraphicsPipeline, StateMode,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    shader::ShaderModule,
    sync::GpuFuture,
};

use super::{
    camera::CameraViewport,
    mesh::{PbrMaterial, PbrSurfaceBuffers, PbrVertex, Surface, Vertex},
    Scene,
};

const IMAGE_FORMAT: Format = Format::B8G8R8A8_UNORM;

pub struct PbrScene {
    vs: Arc<ShaderModule>,
    fs: Arc<ShaderModule>,
    clear_color: [f32; 4],
    render_pass: Arc<RenderPass>,
    pipeline: Arc<GraphicsPipeline>,
    subpass: Subpass,
    viewport: CameraViewport,
    geometry: PbrSurfaceBuffers,
    images: PbrSceneImages,
    msaa_samples: SampleCount,
}
impl PbrScene {
    pub fn new<'a>(
        clear_color: [f32; 4],
        resources: &RenderResources<'a>,
        msaa_samples: SampleCount,
    ) -> Self {
        let device = resources.queue.device().clone();

        let vs = vs::load(device.clone()).unwrap();
        let fs = fs::load(device.clone()).unwrap();

        let buffers = Surface::new(
            1,
            [
                Vertex::new(-0.9, -0.9, 0.5),
                Vertex::new(-0.9, 0.9, 0.5),
                Vertex::new(0.9, -0.9, 0.5),
                Vertex::new(0.8, 0.8, 0.5),
            ],
            [0, 1, 2, 2, 1, 3],
        )
        .as_pbr(
            &PbrMaterial {
                albedo: [0.0, 0.0, 1.0, 1.0],
            },
            &resources.memory_allocator,
        );

        let viewport = CameraViewport::zero();

        let (render_pass, images, subpass, pipeline) =
            Self::create_pipeline(vs.clone(), fs.clone(), resources, msaa_samples, &viewport);

        Self {
            vs,
            fs,
            clear_color,
            render_pass,
            pipeline,
            subpass,
            geometry: buffers,
            viewport,
            images,
            msaa_samples,
        }
    }

    fn create_pipeline<'a>(
        vs: Arc<ShaderModule>,
        fs: Arc<ShaderModule>,
        resources: &RenderResources<'a>,
        msaa_samples: SampleCount,
        viewport: &CameraViewport,
    ) -> (
        Arc<RenderPass>,
        PbrSceneImages,
        Subpass,
        Arc<GraphicsPipeline>,
    ) {
        let device = resources.queue.device();
        let render_pass = {
            if msaa_samples == SampleCount::Sample1 {
                vulkano::single_pass_renderpass!(
                    device.clone(),
                    attachments: {
                        color: {
                            load: DontCare,
                            store: Store,
                            format: IMAGE_FORMAT,
                            samples: 1,
                        }
                    },
                    pass: {
                        color: [color],
                        depth_stencil: {}
                    }
                )
                .unwrap()
            } else {
                vulkano::single_pass_renderpass!(
                    device.clone(),
                    attachments: {
                        intermediary: {
                            load: Clear,
                            store: DontCare,
                            format: IMAGE_FORMAT,
                            samples: msaa_samples,
                        },
                        color: {
                            load: DontCare,
                            store: Store,
                            format: IMAGE_FORMAT,
                            samples: 1,
                        }
                    },
                    pass: {
                        color: [intermediary],
                        depth_stencil: {},
                        resolve: [color]
                    }
                )
                .unwrap()
            }
        };

        let images = PbrSceneImages::new(
            render_pass.clone(),
            resources,
            &viewport,
            msaa_samples,
            IMAGE_FORMAT,
        );

        let subpass = Subpass::from(render_pass.clone(), 0).unwrap();

        let pipeline = GraphicsPipeline::start()
            .vertex_input_state(BuffersDefinition::new().vertex::<PbrVertex>())
            .vertex_shader(vs.entry_point("main").unwrap(), ())
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
                    compare_op: StateMode::Fixed(CompareOp::Greater),
                }),
                ..DepthStencilState::default()
            })
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .fragment_shader(fs.entry_point("main").unwrap(), ())
            .render_pass(subpass.clone())
            .build(device.clone())
            .unwrap();

        (render_pass, images, subpass, pipeline)
    }

    fn update_viewport<'a>(&mut self, info: &PaintCallbackInfo, resources: &RenderResources<'a>) {
        let vp = CameraViewport::from_info(info);
        if vp != self.viewport {
            self.viewport = vp;
            self.images = PbrSceneImages::new(
                self.render_pass.clone(),
                resources,
                &self.viewport,
                self.msaa_samples,
                IMAGE_FORMAT,
            )
        }
    }
}
impl Scene for PbrScene {
    fn render<'a>(&mut self, info: &PaintCallbackInfo, resources: &RenderResources<'a>) {
        self.update_viewport(info, resources);

        let mut scene_builder = AutoCommandBufferBuilder::primary(
            resources.command_buffer_allocator,
            resources.queue.queue_family_index(),
            CommandBufferUsage::MultipleSubmit,
        )
        .unwrap();

        scene_builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some(self.clear_color.into()), None],
                    render_area_offset: self.viewport.origin,
                    render_area_extent: self.viewport.dimensions,
                    ..RenderPassBeginInfo::framebuffer(self.images.framebuffer.clone())
                },
                SubpassContents::Inline,
            )
            .unwrap()
            .set_viewport(0, [self.viewport.to_vulkan_viewport()])
            .bind_pipeline_graphics(self.pipeline.clone())
            .bind_vertex_buffers(0, self.geometry.vertex_buffer.clone())
            .bind_index_buffer(self.geometry.index_buffer.clone())
            //.draw(self.buffers.vertex_buffer.len() as u32, 1, 0, 0)
            .draw_indexed(self.geometry.index_buffer.len() as u32, 1, 0, 0, 0)
            .unwrap()
            .end_render_pass()
            .unwrap();

        let command_buffer = scene_builder.build().unwrap();

        let finished = command_buffer.execute(resources.queue.clone()).unwrap();
        finished
            .then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .unwrap();
    }

    fn view(&self) -> Arc<dyn ImageViewAbstract> {
        self.images.view.clone()
    }
}

struct PbrSceneImages {
    framebuffer: Arc<Framebuffer>,
    view: Arc<ImageView<StorageImage>>,
}
impl PbrSceneImages {
    fn new(
        scene_render_pass: Arc<RenderPass>,
        resources: &RenderResources,
        viewport: &CameraViewport,
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
            match viewport.dimensions[0] > 0 {
                true => viewport.dimensions[0],
                false => 1,
            } + viewport.origin[0],
            match viewport.dimensions[1] > 0 {
                true => viewport.dimensions[1],
                false => 1,
            } + viewport.origin[1],
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

        let image = StorageImage::new(
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

        let view = ImageView::new_default(image.clone()).unwrap();

        attachments.push(view.clone());

        let framebuffer = Framebuffer::new(
            scene_render_pass,
            FramebufferCreateInfo {
                attachments,
                ..Default::default()
            },
        )
        .unwrap();

        Self { framebuffer, view }
    }
}

/*
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
struct SceneVertex {
    position: [f32; 2],
    color: [f32; 4],
}
vulkano::impl_vertex!(SceneVertex, position, color);
*/

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
#version 450
layout(location = 0) in vec3 position;
layout(location = 1) in vec4 albedo;

layout(location = 0) out vec4 v_color;
void main() {
    gl_Position = vec4(position, 1.0);
    v_color = albedo;
}"
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "
#version 450
layout(location = 0) in vec4 v_color;

layout(location = 0) out vec4 f_color;

void main() {
    f_color = v_color;
}"
    }
}
