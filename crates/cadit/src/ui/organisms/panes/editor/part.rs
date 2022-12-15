use byteorder::{BigEndian, ReadBytesExt};
use eframe::{
    egui::{self, PointerButton},
    egui_glow,
    egui_glow::glow,
    epaint::{mutex::Mutex, PaintCallback, Pos2},
};
use std::{fmt::Display, io::Cursor, mem::transmute, sync::Arc, time::Duration};
use three_d::renderer::Geometry;

const ROTATION_SENSITIVITY: f32 = 0.007;

#[derive(Debug, Clone, Copy)]
pub enum CameraAngle {
    // Faces
    Front,
    Right,
    Back,
    Left,
    Top,
    Bottom,

    // Edges
    FrontRight,
    BackRight,
    BackLeft,
    FrontLeft,
    FrontTop,
    BackTop,
    FrontBottom,
    BackBottom,
    RightTop,
    LeftTop,
    RightBottom,
    LeftBottom,

    // Corners
    FrontRightTop,
    BackRightTop,
    BackLeftTop,
    FrontLeftTop,
    FrontRightBottom,
    BackRightBottom,
    BackLeftBottom,
    FrontLeftBottom,
}
impl CameraAngle {
    pub fn from_name(name: &str) -> Self {
        match &*name {
            "Front" => Self::Front,
            "Right" => Self::Right,
            "Back" => Self::Back,
            "Left" => Self::Left,
            "Top" => Self::Top,
            "Bottom" => Self::Bottom,
            "FrontRight" => Self::FrontRight,
            "BackRight" => Self::BackRight,
            "BackLeft" => Self::BackLeft,
            "FrontLeft" => Self::FrontLeft,
            "FrontTop" => Self::FrontTop,
            "BackTop" => Self::BackTop,
            "FrontBottom" => Self::FrontBottom,
            "BackBottom" => Self::BackBottom,
            "RightTop" => Self::RightTop,
            "LeftTop" => Self::LeftTop,
            "RightBottom" => Self::RightBottom,
            "LeftBottom" => Self::LeftBottom,
            "FrontRightTop" => Self::FrontRightTop,
            "BackRightTop" => Self::BackRightTop,
            "BackLeftTop" => Self::BackLeftTop,
            "FrontLeftTop" => Self::FrontLeftTop,
            "FrontRightBottom" => Self::FrontRightBottom,
            "BackRightBottom" => Self::BackRightBottom,
            "BackLeftBottom" => Self::BackLeftBottom,
            "FrontLeftBottom" => Self::FrontLeftBottom,
            _ => panic!("Unknown camera position '{}'", name),
        }
    }

    pub fn get_rotation(&self) -> Quaternion<f32> {
        let x: i32 = match self {
            // Faces
            CameraAngle::Front => 0,
            CameraAngle::Right => 0,
            CameraAngle::Back => 0,
            CameraAngle::Left => 0,
            CameraAngle::Top => 270,
            CameraAngle::Bottom => 90,

            // Edges
            CameraAngle::FrontRight => 0,
            CameraAngle::BackRight => 0,
            CameraAngle::BackLeft => 0,
            CameraAngle::FrontLeft => 0,
            CameraAngle::FrontTop => -45,
            CameraAngle::BackTop => -45,
            CameraAngle::FrontBottom => 45,
            CameraAngle::BackBottom => 45,
            CameraAngle::RightTop => -45,
            CameraAngle::LeftTop => -45,
            CameraAngle::RightBottom => 45,
            CameraAngle::LeftBottom => 45,

            // Corners
            CameraAngle::FrontRightTop => -45,
            CameraAngle::BackRightTop => -45,
            CameraAngle::BackLeftTop => -45,
            CameraAngle::FrontLeftTop => -45,
            CameraAngle::FrontRightBottom => 45,
            CameraAngle::BackRightBottom => 45,
            CameraAngle::BackLeftBottom => 45,
            CameraAngle::FrontLeftBottom => 45,
        };

        let y: i32 = match self {
            // Faces
            CameraAngle::Front => 0,
            CameraAngle::Right => -90,
            CameraAngle::Back => 180,
            CameraAngle::Left => 90,
            CameraAngle::Top => 0,
            CameraAngle::Bottom => 0,

            // Edges
            CameraAngle::FrontRight => -45,
            CameraAngle::BackRight => -135,
            CameraAngle::BackLeft => 135,
            CameraAngle::FrontLeft => 45,
            CameraAngle::FrontTop => 0,
            CameraAngle::BackTop => 180,
            CameraAngle::FrontBottom => 0,
            CameraAngle::BackBottom => 180,
            CameraAngle::RightTop => -90,
            CameraAngle::LeftTop => 90,
            CameraAngle::RightBottom => -90,
            CameraAngle::LeftBottom => 90,

            // Corners
            CameraAngle::FrontRightTop => -45,
            CameraAngle::BackRightTop => -135,
            CameraAngle::BackLeftTop => 135,
            CameraAngle::FrontLeftTop => 45,
            CameraAngle::FrontRightBottom => -45,
            CameraAngle::BackRightBottom => -135,
            CameraAngle::BackLeftBottom => 135,
            CameraAngle::FrontLeftBottom => 45,
        };

        Quaternion::from_angle_x(Deg(x as f32)) * Quaternion::from_angle_y(Deg(y as f32))
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

pub enum ColorOverride {
    Replace(Color),
    Blend(Color, f32),
}
impl ColorOverride {
    pub fn apply_to(&self, original_color: &Color) -> Color {
        match self {
            ColorOverride::Replace(replacement_color) => replacement_color.to_owned(),
            ColorOverride::Blend(target_color, amount) => Color::new(
                Self::interpolate(original_color.r, target_color.r, amount),
                Self::interpolate(original_color.g, target_color.g, amount),
                Self::interpolate(original_color.b, target_color.b, amount),
                Self::interpolate(original_color.a, target_color.a, amount),
            ),
        }
    }

    fn interpolate(from: u8, to: u8, amount: &f32) -> u8 {
        let from = from as f32;
        let to = to as f32;

        ((to - from) * amount + from).round() as u8
    }
}

pub struct PhysicalMaterialOverride {
    pub albedo: Option<ColorOverride>,
    pub albedo_texture: Option<Option<Arc<Texture2D>>>,
    pub metallic: Option<f32>,
    pub roughness: Option<f32>,
    pub metallic_roughness_texture: Option<Option<Arc<Texture2D>>>,
    pub occlusion_strength: Option<f32>,
    pub occlusion_texture: Option<Option<Arc<Texture2D>>>,
    pub normal_scale: Option<f32>,
    pub normal_texture: Option<Option<Arc<Texture2D>>>,
    pub render_states: Option<RenderStates>,
    pub is_transparent: Option<bool>,
    pub emissive: Option<ColorOverride>,
    pub emissive_texture: Option<Option<Arc<Texture2D>>>,
    pub lighting_model: Option<LightingModel>,
}
impl PhysicalMaterialOverride {
    pub fn new() -> Self {
        Self {
            albedo: None,
            albedo_texture: None,
            metallic: None,
            roughness: None,
            metallic_roughness_texture: None,
            occlusion_strength: None,
            occlusion_texture: None,
            normal_scale: None,
            normal_texture: None,
            render_states: None,
            is_transparent: None,
            emissive: None,
            emissive_texture: None,
            lighting_model: None,
        }
    }

    pub fn with_albedo(mut self, albedo: ColorOverride) -> Self {
        self.albedo = Some(albedo);
        self
    }

    pub fn with_albedo_texture(mut self, albedo_texture: Option<Arc<Texture2D>>) -> Self {
        self.albedo_texture = Some(albedo_texture);
        self
    }

    pub fn with_metallic(mut self, metallic: f32) -> Self {
        self.metallic = Some(metallic);
        self
    }

    pub fn with_roughness(mut self, roughness: f32) -> Self {
        self.roughness = Some(roughness);
        self
    }

    pub fn with_metallic_roughness_texture(
        mut self,
        metallic_roughness_texture: Option<Arc<Texture2D>>,
    ) -> Self {
        self.metallic_roughness_texture = Some(metallic_roughness_texture);
        self
    }

    pub fn with_occlusion_strength(mut self, occlusion_strength: f32) -> Self {
        self.occlusion_strength = Some(occlusion_strength);
        self
    }

    pub fn with_occlusion_texture(mut self, occlusion_texture: Option<Arc<Texture2D>>) -> Self {
        self.occlusion_texture = Some(occlusion_texture);
        self
    }

    pub fn with_normal_scale(mut self, normal_scale: f32) -> Self {
        self.normal_scale = Some(normal_scale);
        self
    }

    pub fn with_normal_texture(mut self, normal_texture: Option<Arc<Texture2D>>) -> Self {
        self.normal_texture = Some(normal_texture);
        self
    }

    pub fn with_render_states(mut self, render_states: RenderStates) -> Self {
        self.render_states = Some(render_states);
        self
    }

    pub fn with_is_transparent(mut self, is_transparent: bool) -> Self {
        self.is_transparent = Some(is_transparent);
        self
    }

    pub fn with_emissive(mut self, emissive: ColorOverride) -> Self {
        self.emissive = Some(emissive);
        self
    }

    pub fn with_emissive_texture(mut self, emissive_texture: Option<Arc<Texture2D>>) -> Self {
        self.emissive_texture = Some(emissive_texture);
        self
    }

    pub fn with_lighting_model(mut self, lighting_model: LightingModel) -> Self {
        self.lighting_model = Some(lighting_model);
        self
    }

    pub fn apply_to(&self, physical_material: &PhysicalMaterial) -> PhysicalMaterial {
        let mut mat = physical_material.to_owned();

        if let Some(ref albedo) = self.albedo {
            mat.albedo = albedo.apply_to(&mat.albedo);
        }

        if let Some(ref albedo_texture) = self.albedo_texture {
            mat.albedo_texture = albedo_texture.to_owned();
        }

        if let Some(metallic) = self.metallic {
            mat.metallic = metallic;
        }

        if let Some(roughness) = self.roughness {
            mat.roughness = roughness;
        }

        if let Some(ref metallic_roughness_texture) = self.metallic_roughness_texture {
            mat.metallic_roughness_texture = metallic_roughness_texture.to_owned();
        }

        if let Some(occlusion_strength) = self.occlusion_strength {
            mat.occlusion_strength = occlusion_strength;
        }

        if let Some(ref occlusion_texture) = self.occlusion_texture {
            mat.occlusion_texture = occlusion_texture.to_owned();
        }

        if let Some(normal_scale) = self.normal_scale {
            mat.normal_scale = normal_scale;
        }

        if let Some(ref normal_texture) = self.normal_texture {
            mat.normal_texture = normal_texture.to_owned();
        }

        if let Some(render_states) = self.render_states {
            mat.render_states = render_states;
        }

        if let Some(is_transparent) = self.is_transparent {
            mat.is_transparent = is_transparent;
        }

        if let Some(ref emissive) = self.emissive {
            mat.emissive = emissive.apply_to(&mat.emissive);
        }

        if let Some(ref emissive_texture) = self.emissive_texture {
            mat.emissive_texture = emissive_texture.to_owned();
        }

        if let Some(lighting_model) = self.lighting_model {
            mat.lighting_model = lighting_model;
        }

        mat
    }
}

pub struct Palette {
    hover_color: Color,
    select_color: Color,
}
impl Palette {
    fn physical_material_override(
        &self,
        hovered: bool,
        selected: bool,
    ) -> PhysicalMaterialOverride {
        let mut mat = PhysicalMaterialOverride::new();

        if hovered {
            mat.emissive = Some(ColorOverride::Replace(self.hover_color))
        }

        if selected {
            mat.emissive = Some(ColorOverride::Replace(self.select_color));
        }

        mat
    }
}
impl Default for Palette {
    fn default() -> Self {
        Self {
            hover_color: Color::new(0, 135, 215, 255),
            select_color: Color::new(255, 75, 0, 255),
        }
    }
}

pub(crate) struct SceneObjectProps {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ColorId(u32);
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
impl Display for ColorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}
impl Default for ColorId {
    fn default() -> Self {
        Self::none()
    }
}
impl FromCpuMaterial for ColorId {
    fn from_cpu_material(_context: &Context, _cpu_material: &CpuMaterial) -> Self {
        Self::default()
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

pub(crate) struct ColorIdSource {
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

#[derive(Clone, Debug)]
pub struct PointerButtonDown {
    pos: Pos2,
    button: PointerButton,
    down_at: std::time::Instant,
    modifiers: eframe::egui::Modifiers,
    scene_object: Option<ColorId>,
}

pub struct PartEditor {
    id_source: ColorIdSource,
    rotation: AnimatedValue<Quaternion<f32>>,
    scene: Arc<Mutex<Scene>>,
    scene_rect: egui::Rect,
    pointer_buttons_down: Vec<PointerButtonDown>,
    clicked: Option<ColorId>,
}
impl PartEditor {
    pub fn new(gl: GlowContext) -> Self {
        let mut id_source = ColorIdSource::new();

        // Interpolate quaternions:
        /*
        let a = CameraPosition::FrontLeft.get_rotation();
        let b = CameraPosition::Front.get_rotation();
        let c = (a + b) / 2.0;
        */

        Self {
            //rotation: Quaternion::from_axis_angle((0.0, 1.0, 0.0).into(), Rad(0.0)),
            rotation: AnimatedValue::new(CameraAngle::FrontRightTop.get_rotation()),
            scene: Arc::new(Mutex::new(Scene::new(gl.clone(), &mut id_source))),
            id_source,
            scene_rect: egui::Rect {
                min: (0.0, 0.0).into(),
                max: (0.0, 0.0).into(),
            },
            pointer_buttons_down: Vec::new(),
            clicked: None,
        }
    }

    fn ui_pos_to_fbo_pos(&self, ui: &egui::Ui, ui_pos: Pos2) -> Pos2 {
        let pix_per_pt = ui.input().pixels_per_point;
        let x = (ui_pos.x - self.scene_rect.min.x) * pix_per_pt;
        let y = (self.scene_rect.max.y - ui_pos.y) * pix_per_pt;
        Pos2 { x, y }
    }
}
impl Editor for PartEditor {
    fn title(&self) -> String {
        "Part editor".to_owned()
    }

    fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation.set_immediate(rotation);
    }

    fn animate_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation
            .push_swing(rotation, Duration::from_millis(500));
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        self.clicked = None;

        egui::Frame::canvas(ui.style()).show(ui, |ui| {
            let (rect, response) = ui.allocate_at_least(ui.available_size(), egui::Sense::drag());

            self.scene_rect = rect;

            let dx = response.drag_delta().x;
            let dy = response.drag_delta().y;

            if dy != 0.0 || dx != 0.0 {
                let rotation = self.rotation.value();
                self.rotation.set_immediate(
                    Quaternion::from_axis_angle(
                        Vector3::new(-dy, dx, 0.0).normalize(),
                        Rad(Vector3::new(dx, dy, 0.0).magnitude() * ROTATION_SENSITIVITY),
                    ) * rotation,
                );
            }

            let scene = self.scene.clone();

            let rotation = self.rotation.value();

            let paint_callback = PaintCallback {
                rect,
                callback: Arc::new(egui_glow::CallbackFn::new(move |info, painter| {
                    let mut scene = scene.lock();
                    let context = &scene.context;
                    let frame_input = FrameInput::new(context, &info, painter);
                    scene.render(frame_input, rotation);
                })),
            };

            let mut scene = self.scene.lock();
            for event in ui.input().events.iter() {
                match event {
                    egui::Event::PointerMoved(pos) => {
                        let obj_id = scene.read_color_id(self.ui_pos_to_fbo_pos(ui, *pos));
                        scene.hover_object(obj_id);
                    }
                    egui::Event::PointerButton {
                        pos,
                        button,
                        pressed,
                        modifiers,
                    } => {
                        let obj_id = scene.read_color_id(self.ui_pos_to_fbo_pos(ui, *pos));

                        if *pressed {
                            self.pointer_buttons_down
                                .retain(|down| down.button != *button);

                            self.pointer_buttons_down.push(PointerButtonDown {
                                pos: *pos,
                                button: *button,
                                down_at: std::time::Instant::now(),
                                modifiers: modifiers.to_owned(),
                                scene_object: obj_id,
                            });
                        } else {
                            if let Some(obj_id) = obj_id {
                                let down = self
                                    .pointer_buttons_down
                                    .iter()
                                    .find(|down| down.scene_object == Some(obj_id));

                                if let Some(down) = down {
                                    let shift_select = down.modifiers.shift && modifiers.shift;

                                    scene.toggle_select_object(Some(obj_id), !shift_select);

                                    if scene.get_object(Some(obj_id)).is_some() {
                                        self.clicked = Some(obj_id);
                                    }
                                }
                            } else {
                                if !modifiers.shift {
                                    scene.deselect_all_objects();
                                }
                            }

                            self.pointer_buttons_down = Vec::new();
                        }
                    }
                    _ => {}
                }
            }

            ui.painter().add(paint_callback);
        });

        if self.rotation.has_queued() {
            ui.ctx().request_repaint();
        }
    }

    fn clicked(&self) -> Option<SceneObjectProps> {
        let mut scene = self.scene.lock();
        scene.get_object(self.clicked).map(|obj| obj.props())
    }
}

///
/// Translates from egui input to three-d input
///
pub struct FrameInput<'a> {
    screen: three_d::RenderTarget<'a>,
    viewport: three_d::Viewport,
}

impl FrameInput<'_> {
    pub fn new(
        context: &three_d::Context,
        info: &egui::PaintCallbackInfo,
        painter: &egui_glow::Painter,
    ) -> Self {
        use three_d::*;

        // Disable sRGB textures for three-d
        unsafe {
            use glow::HasContext as _;
            context.disable(glow::FRAMEBUFFER_SRGB);
        }

        // Constructs a screen render target to render the final image to
        let viewport = info.viewport_in_pixels();
        let screen = painter.intermediate_fbo().map_or_else(
            || {
                RenderTarget::screen(
                    context,
                    viewport.width_px.round() as u32,
                    viewport.height_px.round() as u32,
                )
            },
            |fbo| {
                RenderTarget::from_framebuffer(
                    context,
                    viewport.width_px.round() as u32,
                    viewport.height_px.round() as u32,
                    fbo,
                )
            },
        );

        // Set where to paint
        let viewport = Viewport {
            x: viewport.left_px.round() as _,
            y: viewport.from_bottom_px.round() as _,
            width: viewport.width_px.round() as _,
            height: viewport.height_px.round() as _,
        };

        Self { screen, viewport }
    }
}

///
/// Based on the `three-d` [Triangle example](https://github.com/asny/three-d/blob/master/examples/triangle/src/main.rs).
/// This is where you'll need to customize
///
use three_d::*;

use crate::{
    error::CaditResult,
    ui::{
        math::{AnimatedValue, Animation},
        GlowContext,
    },
};

use super::Editor;
pub struct Scene {
    context: Context,
    camera: Camera,
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
    fn new(gl: std::sync::Arc<glow::Context>, id_source: &mut ColorIdSource) -> Self {
        let context = Context::from_gl_context(gl).unwrap();

        // Create a camera
        let position = vec3(0.0, 0.0, -15.0); // Camera position
                                              //let position = CameraPosition::Front.get_position() * 15.0;
        let target = vec3(0.0, 0.0, 0.0); // Look-at point
        let dist = (position - target).magnitude(); // Distance from camera origin to look-at point
        let fov_y = degrees(45.0); // Y-FOV for perspective camera
        let height = (fov_y / 2.0).tan() * dist * 2.0; // FOV-equivalent height for ortho camera

        // Ortho camera
        let camera = Camera::new_orthographic(
            Viewport::new_at_origo(1, 1),
            position,
            target,
            vec3(0.0, 1.0, 0.0),
            height,
            0.1,
            1000.0,
        );

        // Perspective camera
        /*
        let camera = Camera::new_perspective(
            Viewport::new_at_origo(1, 1),
            position,
            target,
            vec3(0.0, 1.0, 0.0),
            fov_y,
            0.1,
            1000.0,
        );
        */

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
            context,
            camera,
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

    pub(crate) fn hover_object(&mut self, id: Option<ColorId>) {
        self.objects.iter_mut().for_each(|obj| {
            obj.hovered = if let Some(id) = id && id == obj.id && obj.hoverable { true } else { false };
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
        rotation: Quaternion<f32>,
    ) -> Option<glow::Framebuffer> {
        // Ensure the camera viewport size matches the current window viewport
        // size which changes if the window is resized
        self.camera.set_viewport(Viewport {
            x: 0,
            y: 0,
            width: frame_input.viewport.width,
            height: frame_input.viewport.height,
        });

        for model in self.objects.iter_mut() {
            model.set_transformation(Mat4::from(rotation));
        }

        self.render_pbr(&frame_input);
        self.render_id_textures(&frame_input);

        frame_input.screen.copy_partially_from(
            frame_input.viewport.into(),
            /*
            ColorTexture::Single(&self.id_color_texture),
            DepthTexture::Single(&self.id_depth_texture),
            */
            ColorTexture::Single(&self.pbr_fxaa_color_texture),
            DepthTexture::Single(&self.pbr_fxaa_depth_texture),
            frame_input.viewport,
            WriteMask::default(),
        );

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
        .clear(ClearState::default())
        .render(
            &self.camera,
            &self.objects.physical_render_objects(&self.palette),
            &lights,
        );

        // Apply FXAA
        RenderTarget::new(
            self.pbr_fxaa_color_texture.as_color_target(None),
            self.pbr_fxaa_depth_texture.as_depth_target(),
        )
        .clear(ClearState::default())
        .write(|| {
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
        .render(&self.camera, &self.objects.id_render_objects(), &[]);
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
