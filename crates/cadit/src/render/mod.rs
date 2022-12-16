use std::sync::Arc;

use eframe::{egui, egui_glow, glow};
use three_d::{Color, LightingModel, PhysicalMaterial, RenderStates, Texture2D};

///
/// Translates from egui input to three-d input
///
pub struct FrameInput<'a> {
    pub screen: three_d::RenderTarget<'a>,
    pub viewport: three_d::Viewport,
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
    pub fn physical_material_override(
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
