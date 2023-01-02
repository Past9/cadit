use std::{f32::consts::PI, sync::Arc};

use cgmath::{Deg, InnerSpace, Rad, Zero};
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
            viewport::{Scissor, Viewport, ViewportState},
        },
        GraphicsPipeline, Pipeline, StateMode,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    shader::ShaderModule,
    sync::GpuFuture,
};

use super::{
    camera::Camera,
    cgmath_types::*,
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
    camera: Camera,
    geometry: PbrSurfaceBuffers,
    images: PbrSceneImages,
    msaa_samples: SampleCount,
    scissor: Scissor,
    rotation: Quat,
    position: Vec3,
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
                Vertex::new(-0.9, -0.9, 0.0),
                Vertex::new(-0.9, 0.9, 0.0),
                Vertex::new(0.9, -0.9, 0.0),
                Vertex::new(0.6, 0.6, 0.0),
            ],
            [0, 1, 2, 2, 1, 3],
        )
        .as_pbr(
            &PbrMaterial {
                albedo: [0.0, 0.0, 1.0, 1.0],
            },
            &resources.memory_allocator,
        );

        let scissor = Scissor {
            origin: [0, 0],
            dimensions: [0, 0],
        };

        /*
        let camera = Camera::create_orthographic(
            scissor.dimensions,
            point3(0.0, 0.0, -5.0),
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, -1.0, 0.0).normalize(),
            2.0,
            1.0,
            11.0,
        );
        */

        let camera = Camera::create_perspective(
            scissor.dimensions,
            point3(0.0, 0.0, -3.0),
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, -1.0, 0.0).normalize(),
            Deg(70.0).into(),
            1.0,
            1000.0,
        );

        let (render_pass, images, subpass, pipeline) =
            Self::create_pipeline(vs.clone(), fs.clone(), resources, msaa_samples, &scissor);

        Self {
            vs,
            fs,
            clear_color,
            render_pass,
            pipeline,
            subpass,
            geometry: buffers,
            camera,
            images,
            msaa_samples,
            scissor,
            rotation: Quat::zero(),
            position: Vec3::zero(),
        }
    }

    fn create_pipeline<'a>(
        vs: Arc<ShaderModule>,
        fs: Arc<ShaderModule>,
        resources: &RenderResources<'a>,
        msaa_samples: SampleCount,
        scissor: &Scissor,
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
            &scissor,
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
        let vpip = info.viewport_in_pixels();
        let new_scissor = Scissor {
            origin: [vpip.left_px as u32, vpip.top_px as u32],
            dimensions: [vpip.width_px as u32, vpip.height_px as u32],
        };

        if new_scissor != self.scissor {
            self.scissor = new_scissor;
            self.camera.set_viewport_in_pixels(self.scissor.dimensions);
            self.images = PbrSceneImages::new(
                self.render_pass.clone(),
                resources,
                &self.scissor,
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

        let push_constants = vs::ty::PushConstants {
            view_matrix: self.camera.view_matrix().clone().into(),
            //model_matrix: (Mat4::from(self.rotation) * Mat4::from_translation(self.position)).into(),
            model_matrix: (Mat4::from_translation(self.position) * Mat4::from(self.rotation))
                .into(),
            perspective_matrix: self.camera.perspective_matrix().clone().into(),
        };

        scene_builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some(self.clear_color.into()), None],
                    render_area_offset: self.scissor.origin,
                    render_area_extent: self.scissor.dimensions,
                    ..RenderPassBeginInfo::framebuffer(self.images.framebuffer.clone())
                },
                SubpassContents::Inline,
            )
            .unwrap()
            //.set_viewport(0, [self.scissor().to_vulkan_viewport()])
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
            .bind_pipeline_graphics(self.pipeline.clone())
            .bind_vertex_buffers(0, self.geometry.vertex_buffer.clone())
            .bind_index_buffer(self.geometry.index_buffer.clone())
            .push_constants(self.pipeline.layout().clone(), 0, push_constants)
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

    fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
    }

    fn set_position(&mut self, position: Vec3) {
        self.position = position;
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

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
#version 450
layout(location = 0) in vec3 position;
layout(location = 1) in vec4 albedo;

layout(push_constant) uniform PushConstants {
    mat4 view_matrix;
    mat4 model_matrix;
    mat4 perspective_matrix;
} push_constants;

layout(location = 0) out vec4 v_color;
void main() {
    mat4 model_view_matrix = push_constants.view_matrix * push_constants.model_matrix;
    gl_Position = push_constants.perspective_matrix * model_view_matrix * vec4(position, 1.0);
    //gl_Position = model_view_matrix * vec4(position, 1.0);
    v_color = albedo;
}",
        types_meta: {
            #[derive(Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
        },
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
