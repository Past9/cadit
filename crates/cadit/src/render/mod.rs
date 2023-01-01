use std::sync::Arc;

use eframe::epaint::PaintCallbackInfo;
use egui_winit_vulkano::RenderResources;
use vulkano::{image::ImageViewAbstract, pipeline::graphics::viewport::Viewport};

mod scene;

trait Scene {
    fn render<'a>(&mut self, info: &PaintCallbackInfo, resources: &RenderResources<'a>);
    fn view(&self) -> Arc<dyn ImageViewAbstract>;
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
