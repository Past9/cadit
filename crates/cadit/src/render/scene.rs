use std::sync::Arc;

use eframe::{egui, egui_glow, epaint::Pos2, glow};
use three_d::{
    degrees, vec3, AmbientLight, Angle, AxisAlignedBoundingBox, Camera, ClearState, Color,
    ColorTexture, Context, CpuModel, DepthTexture, DepthTexture2D, FromCpuMaterial, FxaaEffect,
    Geometry, Gm, InnerSpace, Interpolation, Light, LightingModel, Mat4, Material, MaterialType,
    Mesh, PhysicalMaterial, PostMaterial, Quaternion, RenderStates, RenderTarget, RendererError,
    ScissorBox, Texture2D, Viewport, Wrapping, WriteMask,
};

use crate::error::CaditResult;

use super::color_id::{ColorId, ColorIdSource};

pub(crate) struct Scene {
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
    pub fn new(gl: std::sync::Arc<glow::Context>, id_source: &mut ColorIdSource) -> Self {
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

pub(crate) struct SceneObjectProps {
    pub id: ColorId,
    pub name: String,
}
impl SceneObjectProps {
    pub fn new(id: ColorId, name: String) -> Self {
        Self { id, name }
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
