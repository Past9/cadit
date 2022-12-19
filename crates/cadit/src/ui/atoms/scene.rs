use std::{io::Cursor, mem::transmute};

use byteorder::{BigEndian, ReadBytesExt};
use eframe::{epaint::Pos2, glow};
use three_d::{
    degrees, vec3, AmbientLight, Angle, AxisAlignedBoundingBox, Blend, Camera, ClearState, Color,
    ColorTexture, Context, CpuMaterial, CpuModel, Cull, Deg, DepthTest, DepthTexture,
    DepthTexture2D, FromCpuMaterial, FxaaEffect, Geometry, Gm, InnerSpace, Interpolation, Light,
    Mat3, Mat4, Material, MaterialType, Mesh, PhysicalMaterial, Point3, PostMaterial, Program,
    Quaternion, RenderStates, RenderTarget, RendererError, ScissorBox, Texture2D, Vec2, Vec3,
    Vector3, Viewport, Wrapping, WriteMask,
};

use crate::{
    error::CaditResult,
    render::{FrameInput, Palette, PhysicalMaterialOverride},
};

use super::camera::{CameraMode, CameraProps};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorId(u32);
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

        // Create a camera
        /*
        let position = vec3(0.0, 0.0, -15.0); // Camera position
                                              //let position = CameraPosition::Front.get_position() * 15.0;
        let target = vec3(0.0, 0.0, 0.0); // Look-at point
        let dist = (position - target).magnitude(); // Distance from camera origin to look-at point
        let fov_y = degrees(45.0); // Y-FOV for perspective camera
        let height = (fov_y / 2.0).tan() * dist * 2.0; // FOV-equivalent height for ortho camera

        // Ortho camera
        /*
        let camera = Camera::new_orthographic(
            Viewport::new_at_origo(1, 1),
            position,
            target,
            vec3(0.0, 1.0, 0.0),
            height,
            0.1,
            1000.0,
        );
        */

        // Perspective camera
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
        &self.context
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
            &self.camera_props.camera(),
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
