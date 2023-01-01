use bytemuck::{Pod, Zeroable};
use cgmath::{Quaternion, Vector2};
use eframe::epaint::{PaintCallbackInfo, Pos2};
use egui_winit_vulkano::{CallbackContext, RenderResources};
use std::sync::Arc;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer, TypedBufferAccess},
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, PrimaryCommandBufferAbstract,
        RenderPassBeginInfo, SubpassContents,
    },
    descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet},
    format::Format,
    image::{
        view::ImageView, AttachmentImage, ImageDimensions, ImageViewAbstract, SampleCount,
        StorageImage,
    },
    pipeline::{
        graphics::{
            color_blend::{AttachmentBlend, BlendFactor, BlendOp, ColorBlendState},
            depth_stencil::{CompareOp, DepthState, DepthStencilState},
            input_assembly::{InputAssemblyState, PrimitiveTopology},
            multisample::MultisampleState,
            rasterization::{CullMode, FrontFace, RasterizationState},
            vertex_input::BuffersDefinition,
            viewport::{Viewport, ViewportState},
        },
        GraphicsPipeline, Pipeline, PipelineBindPoint, StateMode,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    sync::GpuFuture,
};

use self::egui_fs::ty::PushConstants;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorId(u32);

pub(crate) struct SceneObject {}
impl SceneObject {
    pub fn props(&self) -> SceneObjectProps {
        todo!()
    }
}
pub struct SceneObjectProps {
    pub name: String,
}

/*
impl ColorId {
    fn none() -> Self {
        Self(0)
    }

    fn to_color(&self) -> Color {
        let bytes: [u8; 4] = unsafe { transmute(self.0.to_be()) };
        Color::new(bytes[1], bytes[2], bytes[3], 255)
    }

    fn from_color(color: &Color) -> Self {
        let mut reader = Cursor::new(vec![0, color.r, color.g, color.b]);
        Self(reader.read_u32::<BigEndian>().unwrap())
    }

    fn from_u32(id: u32) -> Self {
        Self(id)
    }
}
impl std::fmt::Display for ColorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}
impl Default for ColorId {
    fn default() -> Self {
        Self::none()
    }
}
impl Material for ColorId {
    fn fragment_shader_source(&self, _use_vertex_colors: bool, _lights: &[&dyn Light]) -> String {
        r#"
            uniform vec4 surfaceColor;

            layout (location = 0) out vec4 outColor;

            void main()
            {
                outColor = surfaceColor;
            }
        "#
        .to_owned()
    }

    fn use_uniforms(&self, program: &Program, _camera: &Camera, _lights: &[&dyn Light]) {
        program.use_uniform("surfaceColor", self.to_color());
    }

    fn render_states(&self) -> RenderStates {
        RenderStates {
            write_mask: WriteMask::COLOR_AND_DEPTH,
            depth_test: DepthTest::Less,
            blend: Blend::Disabled,
            cull: Cull::Back,
        }
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}

pub struct ColorIdSource {
    seed: u32,
}
impl ColorIdSource {
    pub fn new() -> Self {
        Self { seed: 0 }
    }

    pub fn next(&mut self) -> ColorId {
        self.seed += 1;
        ColorId::from_u32(self.seed)
    }
}

pub struct SceneObjectProps {
    pub id: ColorId,
    pub name: String,
}
impl SceneObjectProps {
    pub fn new(id: ColorId, name: String) -> Self {
        Self { id, name }
    }
}

pub(crate) struct SceneObject {
    id: ColorId,
    name: String,

    selectable: bool,
    selected: bool,

    hovered: bool,
    hoverable: bool,

    clickable: bool,

    geometry: Mesh,
    physical_material: PhysicalMaterial,
    physical_material_override: PhysicalMaterialOverride,
}
impl SceneObject {
    pub fn props(&self) -> SceneObjectProps {
        SceneObjectProps::new(self.id, self.name.clone())
    }

    pub(crate) fn from_cpu_model(
        context: &Context,
        id_source: &mut ColorIdSource,
        cpu_model: &CpuModel,
    ) -> CaditResult<Vec<Self>> {
        let mut materials = std::collections::HashMap::new();
        for material in cpu_model.materials.iter() {
            materials.insert(
                material.name.clone(),
                PhysicalMaterial::from_cpu_material(context, material),
            );
        }
        let mut models = Vec::new();
        for trimesh in cpu_model.geometries.iter() {
            let id = id_source.next();
            models.push(if let Some(material_name) = &trimesh.material_name {
                Self {
                    id,
                    name: trimesh.name.clone(),

                    selectable: false,
                    selected: false,

                    clickable: false,

                    hovered: false,
                    hoverable: false,

                    geometry: Mesh::new(context, trimesh),
                    physical_material: materials
                        .get(material_name)
                        .ok_or(RendererError::MissingMaterial(
                            material_name.clone(),
                            trimesh.name.clone(),
                        ))?
                        .clone(),
                    physical_material_override: PhysicalMaterialOverride::new(),
                }
            } else {
                Self {
                    id,
                    name: trimesh.name.clone(),

                    selectable: false,
                    selected: false,

                    clickable: false,

                    hovered: false,
                    hoverable: false,

                    geometry: Mesh::new(context, trimesh),
                    physical_material: PhysicalMaterial::default(),
                    physical_material_override: PhysicalMaterialOverride::new(),
                }
            });
        }

        Ok(models)
    }

    fn physical_render_object(&self, palette: &Palette) -> Gm<&three_d::Mesh, PhysicalMaterial> {
        let material = self
            .physical_material_override
            .apply_to(&self.physical_material);

        Gm {
            geometry: &self.geometry,
            material: palette
                .physical_material_override(self.hovered, self.selected)
                .apply_to(&material),
        }
    }

    fn id_render_object(&self) -> Gm<&three_d::Mesh, &ColorId> {
        Gm {
            geometry: &self.geometry,
            material: &self.id,
        }
    }

    pub fn set_transformation(&mut self, transformation: Mat4) {
        self.geometry.set_transformation(transformation);
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

trait SceneObjects {
    fn find_by_id(&self, id: ColorId) -> Option<&SceneObject>;
    fn find_by_id_mut(&mut self, id: ColorId) -> Option<&mut SceneObject>;
    fn physical_render_objects(
        &self,
        palette: &Palette,
    ) -> Vec<Gm<&three_d::Mesh, PhysicalMaterial>>;
    fn id_render_objects(&self) -> Vec<Gm<&three_d::Mesh, &ColorId>>;
}
impl SceneObjects for Vec<SceneObject> {
    fn physical_render_objects(
        &self,
        palette: &Palette,
    ) -> Vec<Gm<&three_d::Mesh, PhysicalMaterial>> {
        self.iter()
            .map(|m| m.physical_render_object(palette))
            .collect::<Vec<_>>()
    }

    fn id_render_objects(&self) -> Vec<Gm<&three_d::Mesh, &ColorId>> {
        self.iter()
            .map(|m| m.id_render_object())
            .collect::<Vec<_>>()
    }

    fn find_by_id(&self, id: ColorId) -> Option<&SceneObject> {
        self.iter().find(|obj| obj.id == id)
    }

    fn find_by_id_mut(&mut self, id: ColorId) -> Option<&mut SceneObject> {
        self.iter_mut().find(|obj| obj.id == id)
    }
}
impl Geometry for SceneObject {
    fn render_with_material(
        &self,
        material: &dyn Material,
        camera: &Camera,
        lights: &[&dyn Light],
    ) {
        self.geometry.render_with_material(material, camera, lights)
    }

    fn render_with_post_material(
        &self,
        material: &dyn PostMaterial,
        camera: &Camera,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        self.geometry.render_with_post_material(
            material,
            camera,
            lights,
            color_texture,
            depth_texture,
        )
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.geometry.aabb()
    }
}
impl three_d::Object for SceneObject {
    fn render(&self, camera: &Camera, lights: &[&dyn Light]) {
        self.geometry
            .render_with_material(&self.physical_material, camera, lights)
    }

    fn material_type(&self) -> MaterialType {
        self.physical_material.material_type()
    }
}
*/

const MSAA_SAMPLES: SampleCount = SampleCount::Sample8;
const IMAGE_FORMAT: Format = Format::B8G8R8A8_UNORM;

pub struct DeferredScene {
    color: [f32; 4],
    scene: Option<Scene>,
}
impl DeferredScene {
    pub fn empty(color: [f32; 4]) -> Self {
        Self { color, scene: None }
    }

    pub fn require_scene<'a>(&mut self, resources: &RenderResources<'a>) -> &Scene {
        self.scene
            .get_or_insert_with(|| Scene::new(self.color, resources))
    }

    pub fn require_scene_mut<'a>(&mut self, resources: &RenderResources<'a>) -> &mut Scene {
        self.scene
            .get_or_insert_with(|| Scene::new(self.color, resources))
    }

    pub(crate) fn read_color_id(&mut self, pos: Pos2) -> Option<ColorId> {
        if let Some(ref mut scene) = self.scene {
            scene.read_color_id(pos)
        } else {
            None
        }
    }

    pub(crate) fn hover_object(&mut self, id: Option<ColorId>) {
        if let Some(ref mut scene) = self.scene {
            scene.hover_object(id)
        }
    }

    pub(crate) fn toggle_select_object(&mut self, id: Option<ColorId>, exclusive: bool) {
        if let Some(ref mut scene) = self.scene {
            scene.toggle_select_object(id, exclusive);
        }
    }

    pub(crate) fn get_object(&mut self, id: Option<ColorId>) -> Option<&SceneObject> {
        if let Some(ref mut scene) = self.scene {
            scene.get_object(id)
        } else {
            None
        }
    }

    pub(crate) fn deselect_all_objects(&mut self) {
        if let Some(ref mut scene) = self.scene {
            scene.deselect_all_objects();
        }
    }

    pub(crate) fn render(
        &mut self,
        info: &PaintCallbackInfo,
        ctx: &mut CallbackContext,
        model_rotation: Quaternion<f32>,
        camera_position: Vector2<f32>,
    ) {
        self.require_scene_mut(&ctx.resources)
            .render(info, ctx, model_rotation, camera_position);
    }
}

#[derive(PartialEq)]
pub struct SceneViewport {
    origin: [u32; 2],
    dimensions: [u32; 2],
}
impl SceneViewport {
    fn zero() -> Self {
        Self {
            origin: [0, 0],
            dimensions: [0, 0],
        }
    }

    fn from_info(info: &PaintCallbackInfo) -> Self {
        let vp = info.viewport_in_pixels();
        Self {
            origin: [vp.left_px as u32, vp.top_px as u32],
            dimensions: [vp.width_px as u32, vp.height_px as u32],
        }
    }

    fn to_vulkan_viewport(&self) -> Viewport {
        Viewport {
            origin: [self.origin[0] as f32, self.origin[1] as f32],
            dimensions: [self.dimensions[0] as f32, self.dimensions[1] as f32],
            depth_range: 0.0..1.0,
        }
    }
}

pub struct Scene {
    color: [f32; 4],
    scene_render_pass: Arc<RenderPass>,
    scene_pipeline: Arc<GraphicsPipeline>,
    scene_subpass: Subpass,
    scene_viewport: SceneViewport,
    scene_vertex_buffer: Arc<CpuAccessibleBuffer<[SceneVertex]>>,
    scene_images: SceneImages,

    // Egui stuff
    egui_pipeline: Arc<GraphicsPipeline>,
    egui_vertex_buffer: Arc<CpuAccessibleBuffer<[EguiVertex]>>,
}
impl Scene {
    pub fn new<'a>(color: [f32; 4], resources: &RenderResources<'a>) -> Self {
        let (
            scene_render_pass,
            scene_pipeline,
            scene_subpass,
            scene_vertex_buffer,
            scene_viewport,
            scene_images,
        ) = {
            let device = resources.queue.device().clone();
            //let scene_dimensions = [0.0, 0.0];

            let scene_viewport = SceneViewport::zero();

            let scene_render_pass = vulkano::single_pass_renderpass!(
                device.clone(),
                attachments: {
                    intermediary: {
                        load: Clear,
                        store: DontCare,
                        format: IMAGE_FORMAT,
                        samples: MSAA_SAMPLES,
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
            .unwrap();

            let scene_images = SceneImages::new(
                scene_render_pass.clone(),
                resources,
                &scene_viewport,
                MSAA_SAMPLES,
                IMAGE_FORMAT,
            );

            let scene_vertex_buffer = CpuAccessibleBuffer::from_iter(
                &resources.memory_allocator,
                BufferUsage {
                    vertex_buffer: true,
                    ..BufferUsage::empty()
                },
                false,
                [
                    SceneVertex {
                        position: [-0.98, -0.98],
                        color: [1.0, 0.0, 0.0, 1.0],
                    },
                    SceneVertex {
                        position: [-0.98, 0.98],
                        color: [0.0, 1.0, 0.0, 1.0],
                    },
                    SceneVertex {
                        position: [0.98, -0.98],
                        color: [0.0, 0.0, 1.0, 1.0],
                    },
                    SceneVertex {
                        position: [0.98, -0.98],
                        color: [1.0, 0.0, 0.0, 1.0],
                    },
                    SceneVertex {
                        position: [-0.98, 0.98],
                        color: [1.0, 0.0, 0.0, 1.0],
                    },
                    SceneVertex {
                        position: [0.8, 0.9],
                        color: [1.0, 0.0, 0.0, 1.0],
                    },
                ]
                .iter()
                .cloned(),
            )
            .expect("failed to create buffer");

            let scene_subpass = Subpass::from(scene_render_pass.clone(), 0).unwrap();

            let vs = vs::load(device.clone()).unwrap();
            let fs = fs::load(device.clone()).unwrap();

            let scene_pipeline = GraphicsPipeline::start()
                .vertex_input_state(BuffersDefinition::new().vertex::<SceneVertex>())
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
                    rasterization_samples: MSAA_SAMPLES,
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
                .render_pass(scene_subpass.clone())
                .build(device.clone())
                .unwrap();

            (
                scene_render_pass,
                scene_pipeline,
                scene_subpass,
                scene_vertex_buffer,
                scene_viewport,
                scene_images,
            )
        };

        let (egui_pipeline, egui_vertex_buffer) = {
            let egui_vertex_buffer = CpuAccessibleBuffer::from_iter(
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

            let egui_pipeline = {
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
                    .render_pass(subpass.clone())
                    .build(queue.device().clone())
                    .unwrap()
            };

            (egui_pipeline, egui_vertex_buffer)
        };

        Self {
            color,
            scene_render_pass,
            scene_pipeline,
            scene_subpass,
            scene_vertex_buffer,
            scene_viewport,
            scene_images,
            egui_pipeline,
            egui_vertex_buffer,
        }
    }

    fn update_viewport<'a>(&mut self, info: &PaintCallbackInfo, resources: &RenderResources<'a>) {
        let vp = SceneViewport::from_info(info);
        if vp != self.scene_viewport {
            self.scene_viewport = vp;
            self.scene_images = SceneImages::new(
                self.scene_render_pass.clone(),
                resources,
                &self.scene_viewport,
                MSAA_SAMPLES,
                IMAGE_FORMAT,
            )
        }
    }

    fn render_scene<'a>(&mut self, info: &PaintCallbackInfo, resources: &RenderResources<'a>) {
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
                    clear_values: vec![Some([0.0, 0.0, 0.15, 1.0].into()), None],
                    render_area_offset: self.scene_viewport.origin,
                    render_area_extent: self.scene_viewport.dimensions,
                    ..RenderPassBeginInfo::framebuffer(self.scene_images.framebuffer.clone())
                },
                SubpassContents::Inline,
            )
            .unwrap()
            .set_viewport(0, [self.scene_viewport.to_vulkan_viewport()])
            .bind_pipeline_graphics(self.scene_pipeline.clone())
            .bind_vertex_buffers(0, self.scene_vertex_buffer.clone())
            .draw(self.scene_vertex_buffer.len() as u32, 1, 0, 0)
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

    pub(crate) fn render(
        &mut self,
        info: &PaintCallbackInfo,
        ctx: &mut CallbackContext,
        _model_rotation: Quaternion<f32>,
        _camera_position: Vector2<f32>,
    ) {
        self.render_scene(info, &ctx.resources);

        let egui_descriptor_set = PersistentDescriptorSet::new(
            ctx.resources.descriptor_set_allocator,
            self.egui_pipeline
                .layout()
                .set_layouts()
                .get(0)
                .unwrap()
                .clone(),
            [WriteDescriptorSet::image_view(
                0,
                self.scene_images.view.clone() as Arc<dyn ImageViewAbstract>,
            )],
        )
        .unwrap();

        ctx.builder
            .bind_pipeline_graphics(self.egui_pipeline.clone())
            .bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                self.egui_pipeline.layout().clone(),
                0,
                egui_descriptor_set.clone(),
            )
            .push_constants(
                self.egui_pipeline.layout().clone(),
                0,
                PushConstants {
                    color: [1.0, 0.0, 0.0, 1.0],
                },
            )
            .bind_vertex_buffers(0, self.egui_vertex_buffer.clone())
            .draw(self.egui_vertex_buffer.len() as u32, 1, 0, 0)
            .unwrap();
    }

    pub(crate) fn read_color_id(&mut self, _pos: Pos2) -> Option<ColorId> {
        // todo
        None
    }

    pub(crate) fn hover_object(&mut self, _id: Option<ColorId>) {
        // todo
    }

    pub(crate) fn toggle_select_object(&mut self, _id: Option<ColorId>, _exclusive: bool) {
        // todo
    }

    pub(crate) fn get_object(&mut self, _id: Option<ColorId>) -> Option<&SceneObject> {
        // todo
        None
    }

    pub(crate) fn deselect_all_objects(&mut self) {
        // todo
    }
}

struct SceneImages {
    framebuffer: Arc<Framebuffer>,
    intermediary: Arc<ImageView<AttachmentImage>>,
    view: Arc<ImageView<StorageImage>>,
}
impl SceneImages {
    fn new(
        scene_render_pass: Arc<RenderPass>,
        resources: &RenderResources,
        viewport: &SceneViewport,
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

        let intermediary = ImageView::new_default(
            AttachmentImage::multisampled(&resources.memory_allocator, dimensions, samples, format)
                .unwrap(),
        )
        .unwrap();

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

        let framebuffer = Framebuffer::new(
            scene_render_pass,
            FramebufferCreateInfo {
                attachments: vec![intermediary.clone(), view.clone()],
                ..Default::default()
            },
        )
        .unwrap();

        Self {
            framebuffer,
            intermediary,
            view,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
struct SceneVertex {
    position: [f32; 2],
    color: [f32; 4],
}
vulkano::impl_vertex!(SceneVertex, position, color);

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
#version 450
layout(location = 0) in vec2 position;
layout(location = 1) in vec4 color;

layout(location = 0) out vec4 v_color;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
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

// The `color_input` parameter of the `draw` method.
layout(input_attachment_index = 0, set = 0, binding = 0) uniform subpassInput u_diffuse;

layout(push_constant) uniform PushConstants {
    // The `ambient_color` parameter of the `draw` method.
    vec4 color;
} push_constants;

layout(location = 0) out vec4 f_color;

void main() {
    // Load the value at the current pixel.
    vec3 in_diffuse = subpassLoad(u_diffuse).rgb;
    f_color.rgb = in_diffuse; //push_constants.color.rgb * in_diffuse;
    f_color.g = 0.3;
    f_color.a = 1.0;
    //f_color = vec4(1.0, 1.0, 1.0, 1.0);
}",
        types_meta: {
            use bytemuck::{Pod, Zeroable};

            #[derive(Clone, Copy, Zeroable, Pod)]
        },
    }
}

/*
pub struct Scene {
    context: Context,
    camera_props: CameraProps,
    objects: Vec<SceneObject>,
    ambient_lights: Vec<AmbientLight>,

    palette: Palette,

    id_color_texture: Texture2D,
    id_depth_texture: DepthTexture2D,

    pbr_color_texture: Texture2D,
    pbr_depth_texture: DepthTexture2D,
    pbr_fxaa_color_texture: Texture2D,
    pbr_fxaa_depth_texture: DepthTexture2D,
}

impl Scene {
    pub fn new(gl: std::sync::Arc<glow::Context>, id_source: &mut ColorIdSource) -> Self {
        let context = Context::from_gl_context(gl).unwrap();

        let camera_props = CameraProps::new(
            vec3(0.0, 0.0, -15.0),
            vec3(0.0, 0.0, 0.0),
            degrees(45.0),
            CameraMode::Perspective,
            Viewport::new_at_origo(1, 1),
        );

        let mut loaded = three_d_asset::io::load(&["resources/assets/gizmo2.obj"]).unwrap();

        let mut gizmo = SceneObject::from_cpu_model(
            &context,
            id_source,
            &loaded.deserialize("gizmo2.obj").unwrap(),
        )
        .unwrap();

        for obj in gizmo.iter_mut() {
            let is_camera_control = obj.name != "Space";
            obj.clickable = is_camera_control;
            obj.hoverable = is_camera_control;
            obj.selectable = false;
        }

        let (id_color_texture, id_depth_texture) = Self::new_id_textures(&context, 1, 1);

        let (pbr_color_texture, pbr_depth_texture, pbr_fxaa_color_texture, pbr_fxaa_depth_texture) =
            Self::new_physical_textures(&context, 1, 1);

        let ambient_light = AmbientLight::new(&context, 1.0, Color::WHITE);

        Self {
            //context,
            camera_props,
            objects: gizmo,
            ambient_lights: vec![ambient_light],
            id_color_texture,
            id_depth_texture,
            palette: Palette::default(),

            pbr_color_texture,
            pbr_depth_texture,

            pbr_fxaa_color_texture,
            pbr_fxaa_depth_texture,
        }
    }

    pub fn context(&self) -> &Context {
        //&self.context
        todo!()
    }

    pub(crate) fn hover_object(&mut self, id: Option<ColorId>) {
        self.objects.iter_mut().for_each(|obj| {
            obj.hovered = if let Some(id) = id {
                id == obj.id && obj.hoverable
            } else {
                false
            };
        });
    }

    pub(crate) fn toggle_select_object(&mut self, id: Option<ColorId>, exclusive: bool) {
        self.objects.iter_mut().for_each(|obj| {
            if let Some(id) = id {
                if id == obj.id {
                    if obj.selectable {
                        obj.selected = !obj.selected;
                        return;
                    }
                }
            }

            if exclusive {
                obj.selected = false;
            }
        });
    }

    pub(crate) fn get_object(&mut self, id: Option<ColorId>) -> Option<&SceneObject> {
        self.objects.iter().find(|obj| {
            if let Some(id) = id {
                if id == obj.id {
                    return true;
                }
            }

            return false;
        })
    }

    pub(crate) fn get_object_mut(&mut self, id: Option<ColorId>) -> Option<&mut SceneObject> {
        self.objects.iter_mut().find(|obj| {
            if let Some(id) = id {
                if id == obj.id {
                    return true;
                }
            }

            return false;
        })
    }

    pub(crate) fn deselect_all_objects(&mut self) {
        self.objects.iter_mut().for_each(|obj| {
            obj.selected = false;
        });
    }

    pub(crate) fn read_color_id(&mut self, pos: Pos2) -> Option<ColorId> {
        let color = self
            .id_color_texture
            .as_color_target(None)
            .read_partially(ScissorBox {
                x: pos.x as i32,
                y: pos.y as i32,
                width: 1,
                height: 1,
            });

        match color.get(0) {
            Some(color) => {
                let id = ColorId::from_color(color);
                if id.0 > 0 {
                    Some(id)
                } else {
                    None
                }
            }
            None => todo!(),
        }
    }

    pub fn render(
        &mut self,
        frame_input: FrameInput<'_>,
        model_rotation: Quaternion<f32>,
        camera_position: Vec2,
    ) -> Option<glow::Framebuffer> {
        // Ensure the camera viewport size matches the current window viewport
        // size which changes if the window is resized
        self.camera_props.set_viewport(Viewport {
            x: 0,
            y: 0,
            width: frame_input.viewport.width,
            height: frame_input.viewport.height,
        });

        for model in self.objects.iter_mut() {
            let cam_pos_3 = vec3(camera_position.x, camera_position.y, 0.0);
            model.set_transformation(
                Mat4::from_translation(-cam_pos_3) * Mat4::from(model_rotation),
            );
        }

        self.render_pbr(&frame_input);
        self.render_id_textures(&frame_input);

        let color_texture = ColorTexture::Single(&self.pbr_fxaa_color_texture);
        frame_input
            .screen
            .write_partially(frame_input.viewport.into(), || {
                let fragment_shader_source = format!(
                    "{}\nin vec2 uvs;
                    layout (location = 0) out vec4 color;
                    void main()
                    {{
                        color = sample_color(uvs);
                    }}",
                    color_texture.fragment_shader_source()
                );
                apply_effect(
                    &self.context,
                    &fragment_shader_source,
                    RenderStates {
                        depth_test: DepthTest::Always,
                        write_mask: WriteMask::default(),
                        blend: Blend::TRANSPARENCY,
                        ..Default::default()
                    },
                    frame_input.viewport,
                    |program| {
                        color_texture.use_uniforms(program);
                    },
                )
            });

        // Take back the screen fbo, we will continue to use it.
        frame_input.screen.into_framebuffer()
    }

    pub fn render_pbr(&mut self, frame_input: &FrameInput<'_>) {
        if frame_input.viewport.width != self.pbr_color_texture.width()
            || frame_input.viewport.height != self.pbr_color_texture.height()
            || frame_input.viewport.width != self.pbr_fxaa_color_texture.width()
            || frame_input.viewport.height != self.pbr_fxaa_color_texture.height()
        {
            let (
                pbr_color_texture,
                pbr_depth_texture,
                pbr_fxaa_color_texture,
                pbr_fxaa_depth_texture,
            ) = Self::new_physical_textures(
                &self.context,
                frame_input.viewport.width,
                frame_input.viewport.height,
            );

            self.pbr_color_texture = pbr_color_texture;
            self.pbr_depth_texture = pbr_depth_texture;
            self.pbr_fxaa_color_texture = pbr_fxaa_color_texture;
            self.pbr_fxaa_depth_texture = pbr_fxaa_depth_texture;
        }

        let lights = self
            .ambient_lights
            .iter()
            .map(|l| l as &dyn Light)
            .collect::<Vec<&dyn Light>>();

        // Render offscreen
        RenderTarget::new(
            self.pbr_color_texture.as_color_target(None),
            self.pbr_depth_texture.as_depth_target(),
        )
        .clear(ClearState {
            red: None,
            green: None,
            blue: None,
            alpha: Some(0.0),
            depth: Some(1.0),
        })
        .render(
            &self.camera_props.camera(),
            &self.objects.physical_render_objects(&self.palette),
            &lights,
        );

        // Apply FXAA
        RenderTarget::new(
            self.pbr_fxaa_color_texture.as_color_target(None),
            self.pbr_fxaa_depth_texture.as_depth_target(),
        )
        .write(|| {
            // TODO: This causes artifacts (dark pixels) around edges when rendered on transparent
            // background, possibly because the default fxaa_effect.frag shader only blends the RGB
            // values and ignores the alpha. Try writing a custom FXAA shader that blends the alpha
            // channel as well.
            (FxaaEffect {}).apply(&self.context, ColorTexture::Single(&self.pbr_color_texture));
        });
    }

    pub fn render_id_textures(&mut self, frame_input: &FrameInput<'_>) {
        if frame_input.viewport.width != self.id_color_texture.width()
            || frame_input.viewport.height != self.id_color_texture.height()
        {
            let (id_color_texture, id_depth_texture) = Self::new_id_textures(
                &self.context,
                frame_input.viewport.width,
                frame_input.viewport.height,
            );

            self.id_color_texture = id_color_texture;
            self.id_depth_texture = id_depth_texture;
        }

        // Render offscreen
        RenderTarget::new(
            self.id_color_texture.as_color_target(None),
            self.id_depth_texture.as_depth_target(),
        )
        .clear(ClearState::default())
        .render(
            &self.camera_props.camera(),
            &self.objects.id_render_objects(),
            &[],
        );
    }

    fn new_id_textures(context: &Context, width: u32, height: u32) -> (Texture2D, DepthTexture2D) {
        (
            Texture2D::new_empty::<[f32; 3]>(
                context,
                width,
                height,
                Interpolation::Nearest,
                Interpolation::Nearest,
                None,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
            ),
            DepthTexture2D::new::<f32>(
                context,
                width,
                height,
                Wrapping::ClampToEdge,
                Wrapping::ClampToEdge,
            ),
        )
    }

    fn new_physical_textures(
        context: &Context,
        width: u32,
        height: u32,
    ) -> (Texture2D, DepthTexture2D, Texture2D, DepthTexture2D) {
        // Create offscreen textures to render the initial image to
        let pbr_color_texture = Texture2D::new_empty::<[u8; 4]>(
            context,
            width,
            height,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );

        let pbr_depth_texture = DepthTexture2D::new::<f32>(
            context,
            width,
            height,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );

        // Create offscreen textures to apply FXAA to the rendered image. Should be able to
        // apply this directly when copying to the screen, but the coordinates are off
        // because write_partially(...) does not allow specifying source coordinates.
        let pbr_fxaa_color_texture = Texture2D::new_empty::<[u8; 4]>(
            context,
            width,
            height,
            Interpolation::Nearest,
            Interpolation::Nearest,
            None,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );

        let pbr_fxaa_depth_texture = DepthTexture2D::new::<f32>(
            context,
            width,
            height,
            Wrapping::ClampToEdge,
            Wrapping::ClampToEdge,
        );

        (
            pbr_color_texture,
            pbr_depth_texture,
            pbr_fxaa_color_texture,
            pbr_fxaa_depth_texture,
        )
    }
}
*/
