use eframe::epaint::{PaintCallbackInfo, Pos2};
use egui_winit_vulkano::{CallbackContext, RenderResources};
use vulkano::{image::SampleCount, pipeline::graphics::viewport::Viewport};

use crate::render::{
    cgmath_types::{Quat, Vec3},
    egui_transfer::EguiTransfer,
    renderer::Renderer,
};

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

const MSAA_SAMPLES: SampleCount = SampleCount::Sample8;

pub struct DeferredRenderer {
    color: [f32; 4],
    renderer: Option<EguiRenderer>,
}
impl DeferredRenderer {
    pub fn empty(color: [f32; 4]) -> Self {
        Self {
            color,
            renderer: None,
        }
    }

    pub fn require_scene<'a>(&mut self, resources: &RenderResources<'a>) -> &EguiRenderer {
        self.renderer
            .get_or_insert_with(|| EguiRenderer::new(self.color, resources))
    }

    pub fn require_scene_mut<'a>(&mut self, resources: &RenderResources<'a>) -> &mut EguiRenderer {
        self.renderer
            .get_or_insert_with(|| EguiRenderer::new(self.color, resources))
    }

    pub(crate) fn read_color_id(&mut self, pos: Pos2) -> Option<ColorId> {
        if let Some(ref mut renderer) = self.renderer {
            renderer.read_color_id(pos)
        } else {
            None
        }
    }

    pub(crate) fn hover_object(&mut self, id: Option<ColorId>) {
        if let Some(ref mut renderer) = self.renderer {
            renderer.hover_object(id)
        }
    }

    pub(crate) fn toggle_select_object(&mut self, id: Option<ColorId>, exclusive: bool) {
        if let Some(ref mut renderer) = self.renderer {
            renderer.toggle_select_object(id, exclusive);
        }
    }

    pub(crate) fn get_object(&mut self, id: Option<ColorId>) -> Option<&SceneObject> {
        if let Some(ref mut renderer) = self.renderer {
            renderer.get_object(id)
        } else {
            None
        }
    }

    pub(crate) fn deselect_all_objects(&mut self) {
        if let Some(ref mut renderer) = self.renderer {
            renderer.deselect_all_objects();
        }
    }

    pub(crate) fn render(
        &mut self,
        info: &PaintCallbackInfo,
        ctx: &mut CallbackContext,
        model_rotation: Quat,
        camera_position: Vec3,
    ) {
        self.require_scene_mut(&ctx.resources)
            .render(info, ctx, model_rotation, camera_position);
    }

    pub(crate) fn set_rotation(&mut self, rotation: Quat) {
        if let Some(ref mut scene) = self.renderer {
            scene.set_rotation(rotation);
        }
    }

    pub(crate) fn set_position(&mut self, position: Vec3) {
        if let Some(ref mut scene) = self.renderer {
            scene.set_position(position);
        }
    }
}

#[derive(PartialEq)]
pub struct GuiViewport {
    origin: [u32; 2],
    dimensions: [u32; 2],
}
impl GuiViewport {
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

pub struct EguiRenderer {
    renderer: Renderer,
    transfer: EguiTransfer,
}
impl EguiRenderer {
    pub fn new<'a>(color: [f32; 4], resources: &RenderResources<'a>) -> Self {
        let renderer = Renderer::new(color, resources, MSAA_SAMPLES);
        let transfer = EguiTransfer::new(resources);

        Self { renderer, transfer }
    }

    pub(crate) fn render(
        &mut self,
        info: &PaintCallbackInfo,
        ctx: &mut CallbackContext,
        _model_rotation: Quat,
        _camera_position: Vec3,
    ) {
        self.renderer.render(info, &ctx.resources);
        self.transfer.transfer(self.renderer.view(), ctx);
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

    fn set_rotation(&mut self, rotation: Quat) {
        self.renderer.set_rotation(rotation);
    }

    fn set_position(&mut self, position: Vec3) {
        self.renderer.set_position(position);
    }
}
