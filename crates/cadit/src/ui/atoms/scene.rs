use eframe::epaint::{PaintCallbackInfo, Pos2};
use egui_winit_vulkano::{CallbackContext, RenderResources};
use std::sync::Arc;
use vulkano::{
    format::Format,
    image::{view::ImageView, AttachmentImage, ImageDimensions, SampleCount, StorageImage},
    pipeline::graphics::viewport::Viewport,
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
};

use crate::render::{
    cgmath_types::{Quat, Vec3},
    egui_transfer::EguiTransfer,
    pbr_scene::PbrScene,
    Scene,
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

pub struct DeferredScene {
    color: [f32; 4],
    scene: Option<EguiScene>,
}
impl DeferredScene {
    pub fn empty(color: [f32; 4]) -> Self {
        Self { color, scene: None }
    }

    pub fn require_scene<'a>(&mut self, resources: &RenderResources<'a>) -> &EguiScene {
        self.scene
            .get_or_insert_with(|| EguiScene::new(self.color, resources))
    }

    pub fn require_scene_mut<'a>(&mut self, resources: &RenderResources<'a>) -> &mut EguiScene {
        self.scene
            .get_or_insert_with(|| EguiScene::new(self.color, resources))
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
        model_rotation: Quat,
        camera_position: Vec3,
    ) {
        self.require_scene_mut(&ctx.resources)
            .render(info, ctx, model_rotation, camera_position);
    }

    pub(crate) fn set_rotation(&mut self, rotation: Quat) {
        if let Some(ref mut scene) = self.scene {
            scene.set_rotation(rotation);
        }
    }

    pub(crate) fn set_position(&mut self, position: Vec3) {
        if let Some(ref mut scene) = self.scene {
            scene.set_position(position);
        }
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

pub struct EguiScene {
    scene: PbrScene,
    transfer: EguiTransfer,
}
impl EguiScene {
    pub fn new<'a>(color: [f32; 4], resources: &RenderResources<'a>) -> Self {
        let scene = PbrScene::new(color, resources, MSAA_SAMPLES);
        let transfer = EguiTransfer::new(resources);

        Self { scene, transfer }
    }

    pub(crate) fn render(
        &mut self,
        info: &PaintCallbackInfo,
        ctx: &mut CallbackContext,
        _model_rotation: Quat,
        _camera_position: Vec3,
    ) {
        self.scene.render(info, &ctx.resources);
        self.transfer.transfer(self.scene.view(), ctx);
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
        self.scene.set_rotation(rotation);
    }

    fn set_position(&mut self, position: Vec3) {
        self.scene.set_position(position);
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
