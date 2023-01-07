use cgmath::{Deg, InnerSpace};
use eframe::epaint::{PaintCallbackInfo, Pos2};
use egui_winit_vulkano::{CallbackContext, RenderResources};
use vulkano::image::SampleCount;

use render::{
    camera::Camera,
    cgmath_types::{point3, vec3, Point3, Quat, Vec2, Vec3},
    lights::DirectionalLight,
    mesh::{Edge, EdgeVertex, Point, Surface, SurfaceVertex},
    model::{Material, Model, ModelEdge, ModelPoint, ModelSurface},
    renderer::Renderer,
    rgba,
    scene::{Scene, SceneLights},
    PixelViewport, Rgb, Rgba,
};

use super::{egui_transfer::EguiTransfer, ColorId, SceneObject};

pub(super) struct GuiRenderer {
    color: [f32; 4],
    internal: Option<InternalGuiRenderer>,
}
impl GuiRenderer {
    pub(super) fn empty(color: [f32; 4]) -> Self {
        Self {
            color,
            internal: None,
        }
    }

    pub(super) fn camera_vec_to(&self, location: Point3) -> Option<Vec3> {
        if let Some(ref renderer) = self.internal {
            Some(renderer.camera_vec_to(location))
        } else {
            None
        }
    }

    pub(super) fn viewport_size_at_dist(&self, dist: f32) -> Option<Vec2> {
        if let Some(ref renderer) = self.internal {
            Some(renderer.viewport_size_at_dist(dist))
        } else {
            None
        }
    }

    pub(super) fn read_color_id(&mut self, pos: Pos2) -> Option<ColorId> {
        if let Some(ref mut renderer) = self.internal {
            renderer.read_color_id(pos)
        } else {
            None
        }
    }

    pub(super) fn hover_object(&mut self, id: Option<ColorId>) {
        if let Some(ref mut renderer) = self.internal {
            renderer.hover_object(id)
        }
    }

    pub(super) fn toggle_select_object(&mut self, id: Option<ColorId>, exclusive: bool) {
        if let Some(ref mut renderer) = self.internal {
            renderer.toggle_select_object(id, exclusive);
        }
    }

    pub(super) fn get_object(&mut self, id: Option<ColorId>) -> Option<&SceneObject> {
        if let Some(ref mut renderer) = self.internal {
            renderer.get_object(id)
        } else {
            None
        }
    }

    pub(super) fn deselect_all_objects(&mut self) {
        if let Some(ref mut renderer) = self.internal {
            renderer.deselect_all_objects();
        }
    }

    pub(super) fn render(&mut self, info: &PaintCallbackInfo, ctx: &mut CallbackContext) {
        self.require_internal_mut(&ctx.resources).render(info, ctx);
    }

    pub(super) fn set_rotation(&mut self, rotation: Quat) {
        if let Some(ref mut internal) = self.internal {
            internal
                .scene_renderer
                .scene_mut()
                .orientation_mut()
                .set_rotation(rotation);
        }
    }

    pub(super) fn set_position(&mut self, offset: Vec3) {
        if let Some(ref mut internal) = self.internal {
            internal
                .scene_renderer
                .scene_mut()
                .orientation_mut()
                .set_offset(offset);
        }
    }

    fn require_internal<'a>(&mut self, resources: &RenderResources<'a>) -> &InternalGuiRenderer {
        self.internal
            .get_or_insert_with(|| InternalGuiRenderer::new(resources))
    }

    fn require_internal_mut<'a>(
        &mut self,
        resources: &RenderResources<'a>,
    ) -> &mut InternalGuiRenderer {
        self.internal
            .get_or_insert_with(|| InternalGuiRenderer::new(resources))
    }
}

struct InternalGuiRenderer {
    scene_renderer: Renderer,
    transfer: EguiTransfer,
}
impl InternalGuiRenderer {
    fn new<'a>(resources: &RenderResources<'a>) -> Self {
        let renderer = Renderer::new(
            Scene::new(
                rgba(0.0, 0.05, 0.08, 1.0),
                SceneLights::new(
                    vec![
                        //AmbientLight::new(Rgb::WHITE, 0.05),
                        //AmbientLight::new(Rgb::RED, 0.5),
                    ],
                    vec![
                        DirectionalLight::new(vec3(1.0, 0.0, 1.0).normalize(), Rgb::BLUE, 1.0),
                        DirectionalLight::new(vec3(-1.0, 0.0, 1.0).normalize(), Rgb::YELLOW, 1.0),
                    ],
                    vec![
                        //PointLight::new(point3(3.0, 3.0, -5.0), Rgb::RED, 7.0),
                        //PointLight::new(point3(-3.0, -3.0, -5.0), Rgb::GREEN, 2.0),
                    ],
                ),
                Camera::create_perspective(
                    [0, 0],
                    point3(0.0, 0.0, -5.0),
                    vec3(0.0, 0.0, 1.0),
                    vec3(0.0, -1.0, 0.0).normalize(),
                    Deg(70.0).into(),
                    0.01,
                    5.0,
                ),
                vec![Model::new(
                    vec![ModelSurface::new(
                        0.into(),
                        Surface::new(
                            [
                                SurfaceVertex::new(point3(-0.9, -0.9, 0.0), vec3(0.0, 0.0, -1.0)),
                                SurfaceVertex::new(point3(-0.9, 0.9, 0.0), vec3(0.0, 0.0, -1.0)),
                                SurfaceVertex::new(point3(0.9, -0.9, 0.0), vec3(0.0, 0.0, -1.0)),
                                SurfaceVertex::new(point3(0.6, 0.6, 0.0), vec3(0.0, 0.0, -1.0)),
                            ],
                            [0, 1, 2, 2, 1, 3],
                        ),
                        0,
                    )],
                    vec![ModelEdge::new(
                        0.into(),
                        Edge::new([
                            EdgeVertex::new(point3(-0.9, -0.9, 0.0), vec3(0.0, 0.0, -1.0)),
                            EdgeVertex::new(point3(-0.9, 0.9, 0.0), vec3(0.0, 0.0, -1.0)),
                            EdgeVertex::new(point3(0.9, -0.9, 0.0), vec3(0.0, 0.0, -1.0)),
                            EdgeVertex::new(point3(0.6, 0.6, 0.0), vec3(0.0, 0.0, -1.0)),
                        ]),
                        Rgba::BLACK,
                    )],
                    vec![
                        ModelPoint::new(
                            0.into(),
                            Point::new(point3(-0.9, -0.9, 0.0), vec3(0.0, 0.0, -1.0)),
                        ),
                        ModelPoint::new(
                            0.into(),
                            Point::new(point3(-0.9, 0.9, 0.0), vec3(0.0, 0.0, -1.0)),
                        ),
                        ModelPoint::new(
                            0.into(),
                            Point::new(point3(0.9, -0.9, 0.0), vec3(0.0, 0.0, -1.0)),
                        ),
                        ModelPoint::new(
                            0.into(),
                            Point::new(point3(0.6, 0.6, 0.0), vec3(0.0, 0.0, -1.0)),
                        ),
                    ],
                )],
                vec![Material::new(rgba(1.0, 1.0, 1.0, 1.0), 0.5)],
            ),
            SampleCount::Sample8,
            &resources.memory_allocator,
            &resources.descriptor_set_allocator,
            resources.queue.clone(),
        );
        let transfer = EguiTransfer::new(resources);

        Self {
            scene_renderer: renderer,
            transfer,
        }
    }

    fn camera_vec_to(&self, location: Point3) -> Vec3 {
        self.scene_renderer.camera_vec_to(location)
    }

    fn viewport_size_at_dist(&self, dist: f32) -> Vec2 {
        self.scene_renderer.viewport_size_at_dist(dist)
    }

    fn render(&mut self, info: &PaintCallbackInfo, ctx: &mut CallbackContext) {
        let vpip = info.viewport_in_pixels();

        self.scene_renderer.render(
            &PixelViewport {
                left: vpip.left_px as u32,
                top: vpip.top_px as u32,
                width: vpip.width_px as u32,
                height: vpip.height_px as u32,
            },
            &ctx.resources.memory_allocator,
            &ctx.resources.command_buffer_allocator,
            ctx.resources.queue.clone(),
        );
        self.transfer.transfer(self.scene_renderer.view(), ctx);
    }

    fn read_color_id(&mut self, _pos: Pos2) -> Option<ColorId> {
        // todo
        None
    }

    fn hover_object(&mut self, _id: Option<ColorId>) {
        // todo
    }

    fn toggle_select_object(&mut self, _id: Option<ColorId>, _exclusive: bool) {
        // todo
    }

    fn get_object(&mut self, _id: Option<ColorId>) -> Option<&SceneObject> {
        // todo
        None
    }

    fn deselect_all_objects(&mut self) {
        // todo
    }

    fn scene_mut(&mut self) -> &mut Scene {
        self.scene_renderer.scene_mut()
    }

    fn set_rotation(&mut self, rotation: Quat) {
        self.scene_renderer
            .scene_mut()
            .orientation_mut()
            .set_rotation(rotation);
    }

    fn set_offset(&mut self, offset: Vec3) {
        self.scene_renderer
            .scene_mut()
            .orientation_mut()
            .set_offset(offset)
    }
}
