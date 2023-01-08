use cgmath::{point3, vec3, Deg, InnerSpace};
use eframe::egui;
use render::{
    camera::{Camera, CameraAngle},
    lights::DirectionalLight,
    mesh::{Edge, EdgeVertex, Point, Surface, SurfaceVertex},
    model::{Material, Model, ModelEdge, ModelPoint, ModelSurface},
    scene::{Scene, SceneLights},
    Rgb, Rgba,
};
use widgets::{rgba, scene::SceneViewer};
use window::{run_window, Window, WindowDescriptor};

pub fn main() {
    run_window(
        App::new(),
        &WindowDescriptor {
            position: Some([320.0, 50.0]),
            width: 1920.0,
            height: 1080.0,
            ..Default::default()
        },
    )
}

pub struct App {
    viewer: SceneViewer,
}
impl App {
    pub fn new() -> Self {
        Self {
            viewer: SceneViewer::new(
                CameraAngle::Front.get_rotation(),
                vec3(0.0, 0.0, 0.0),
                true,
                true,
                true,
                Scene::new(
                    rgba(0.05, 0.1, 0.15, 1.0),
                    SceneLights::new(
                        vec![],
                        vec![
                            DirectionalLight::new(vec3(1.0, 0.0, 1.0).normalize(), Rgb::BLUE, 1.0),
                            DirectionalLight::new(
                                vec3(-1.0, 0.0, 1.0).normalize(),
                                Rgb::YELLOW,
                                1.0,
                            ),
                        ],
                        vec![],
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
                                    SurfaceVertex::new(
                                        point3(-0.9, -0.9, 0.0),
                                        vec3(0.0, 0.0, -1.0),
                                    ),
                                    SurfaceVertex::new(
                                        point3(-0.9, 0.9, 0.0),
                                        vec3(0.0, 0.0, -1.0),
                                    ),
                                    SurfaceVertex::new(
                                        point3(0.9, -0.9, 0.0),
                                        vec3(0.0, 0.0, -1.0),
                                    ),
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
            ),
        }
    }
}
impl Window for App {
    fn draw(&mut self, gui: &mut window::Gui) {
        gui.immediate_ui(|gui| {
            let ctx = &gui.egui_ctx;
            egui::CentralPanel::default().show(ctx, |ui| {
                self.viewer.show(ui);
            });
        });
    }
}
