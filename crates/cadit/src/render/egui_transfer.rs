use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use egui_winit_vulkano::{CallbackContext, RenderResources};
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer, TypedBufferAccess},
    descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet},
    image::ImageViewAbstract,
    pipeline::{
        graphics::{
            color_blend::ColorBlendState,
            input_assembly::InputAssemblyState,
            rasterization::{CullMode, FrontFace, RasterizationState},
            vertex_input::BuffersDefinition,
            viewport::ViewportState,
        },
        GraphicsPipeline, Pipeline, PipelineBindPoint, StateMode,
    },
};

pub struct EguiTransfer {
    pipeline: Arc<GraphicsPipeline>,
    quad: Arc<CpuAccessibleBuffer<[EguiVertex]>>,
}
impl EguiTransfer {
    pub fn new<'a>(resources: &RenderResources<'a>) -> Self {
        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            &resources.memory_allocator,
            BufferUsage {
                vertex_buffer: true,
                ..Default::default()
            },
            false,
            [
                EguiVertex {
                    position: [-1.0, -1.0],
                },
                EguiVertex {
                    position: [-1.0, 1.0],
                },
                EguiVertex {
                    position: [1.0, -1.0],
                },
                EguiVertex {
                    position: [1.0, -1.0],
                },
                EguiVertex {
                    position: [-1.0, 1.0],
                },
                EguiVertex {
                    position: [1.0, 1.0],
                },
            ],
        )
        .unwrap();

        let pipeline = {
            let queue = resources.queue.clone();
            let subpass = resources.subpass.clone();

            let egui_vs =
                egui_vs::load(queue.device().clone()).expect("failed to create shader module");
            let egui_fs =
                egui_fs::load(queue.device().clone()).expect("failed to create shader module");

            GraphicsPipeline::start()
                .vertex_input_state(BuffersDefinition::new().vertex::<EguiVertex>())
                .vertex_shader(egui_vs.entry_point("main").unwrap(), ())
                .input_assembly_state(InputAssemblyState::new())
                .viewport_state(ViewportState::viewport_dynamic_scissor_dynamic(1))
                .fragment_shader(egui_fs.entry_point("main").unwrap(), ())
                .rasterization_state(RasterizationState {
                    front_face: StateMode::Fixed(FrontFace::CounterClockwise),
                    cull_mode: StateMode::Fixed(CullMode::None),
                    ..Default::default()
                })
                .color_blend_state(
                    ColorBlendState::new(subpass.num_color_attachments()).blend_alpha(),
                )
                /*
                .color_blend_state(ColorBlendState::new(subpass.num_color_attachments()).blend(
                    AttachmentBlend {
                        color_op: BlendOp::Add,
                        color_source: BlendFactor::One,
                        color_destination: BlendFactor::One,
                        alpha_op: BlendOp::Max,
                        alpha_source: BlendFactor::One,
                        alpha_destination: BlendFactor::One,
                    },
                ))
                */
                .render_pass(subpass.clone())
                .build(queue.device().clone())
                .unwrap()
        };

        Self {
            pipeline,
            quad: vertex_buffer,
        }
    }

    pub(crate) fn transfer(&mut self, view: Arc<dyn ImageViewAbstract>, ctx: &mut CallbackContext) {
        let egui_descriptor_set = PersistentDescriptorSet::new(
            ctx.resources.descriptor_set_allocator,
            self.pipeline.layout().set_layouts().get(0).unwrap().clone(),
            [WriteDescriptorSet::image_view(0, view)],
        )
        .unwrap();

        ctx.builder
            .bind_pipeline_graphics(self.pipeline.clone())
            .bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                self.pipeline.layout().clone(),
                0,
                egui_descriptor_set.clone(),
            )
            .bind_vertex_buffers(0, self.quad.clone())
            .draw(self.quad.len() as u32, 1, 0, 0)
            .unwrap();
    }
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
struct EguiVertex {
    position: [f32; 2],
}
vulkano::impl_vertex!(EguiVertex, position);

mod egui_vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
#version 450

layout(location = 0) in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}"
    }
}

mod egui_fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "
#version 450

// The final rendered scene.
layout(input_attachment_index = 0, set = 0, binding = 0) uniform subpassInput u_diffuse;

layout(location = 0) out vec4 f_color;

void main() {
    // Load the value at the current pixel.
    f_color.rgba = subpassLoad(u_diffuse).rgba;
}",
        types_meta: {
            #[derive(Clone, Copy, Zeroable, Pod)]
        },
    }
}
