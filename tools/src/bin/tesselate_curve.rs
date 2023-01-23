use cgmath::{point3, vec3, Deg, InnerSpace};
use components::{rgba, scene::SceneViewer, Gui};
use components::{run_window, Window, WindowDescriptor};
use eframe::egui;
use render::{
    camera::{Camera, CameraAngle},
    lights::DirectionalLight,
    mesh::{Edge, EdgeVertex, Point},
    model::{Material, Model, ModelEdge, ModelPoint},
    scene::{Scene, SceneLights},
    Rgb, Rgba,
};
use space::hspace::HSpace3;
use space::{EVector, HVec3};
use spline::math::FloatRange;
use tesselate::tesselate_bezier_curve;

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
        // Create grid
        let grid_lines = {
            let gs = 5;
            let color = rgba(0.0, 0.075, 0.15, 1.0);
            let grid_lines = (-gs..=gs)
                .map(|x| {
                    ModelEdge::new(
                        0.into(),
                        Edge {
                            vertices: vec![
                                EdgeVertex {
                                    position: [x as f32, gs as f32, 0.0],
                                    expand: [0.0, 0.0, 0.0],
                                },
                                EdgeVertex {
                                    position: [x as f32, -gs as f32, 0.0],
                                    expand: [0.0, 0.0, 0.0],
                                },
                            ],
                        },
                        color,
                    )
                })
                .chain((-gs..=gs).map(|y| {
                    ModelEdge::new(
                        0.into(),
                        Edge {
                            vertices: vec![
                                EdgeVertex {
                                    position: [gs as f32, y as f32, 0.0],
                                    expand: [0.0, 0.0, 0.0],
                                },
                                EdgeVertex {
                                    position: [-gs as f32, y as f32, 0.0],
                                    expand: [0.0, 0.0, 0.0],
                                },
                            ],
                        },
                        color,
                    )
                }))
                .collect::<Vec<_>>();

            grid_lines
        };

        // Create curve
        let curve = spline::nurbs_curve::NurbsCurve::<HSpace3>::example_quarter_circle();

        let num_segments = 5000;
        let reference_edge = ModelEdge::new(
            0.into(),
            Edge {
                vertices: FloatRange::new(curve.min_u(), curve.max_u(), num_segments)
                    .map(|u| EdgeVertex {
                        position: curve.point(u).f32s(),
                        expand: [0.0, 0.0, 0.0],
                    })
                    .collect::<Vec<_>>(),
            },
            Rgba::BLACK,
        );

        let tolerance = 0.1;
        let beziers = curve.decompose();
        let tesselated_edges = beziers
            .iter()
            .map(|bezier| {
                tesselate_bezier_curve(bezier, tolerance).to_model_edge(0.into(), Rgba::GREEN)
            })
            .collect::<Vec<_>>();

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
                        point3(-0.25, -0.5, -1.0),
                        vec3(0.0, 0.0, 1.0),
                        vec3(0.0, -1.0, 0.0).normalize(),
                        Deg(70.0).into(),
                        0.01,
                        5.0,
                    ),
                    vec![Model::new(
                        vec![],
                        vec![reference_edge]
                            .into_iter()
                            .chain(grid_lines.into_iter())
                            .chain(tesselated_edges)
                            .collect(),
                        vec![],
                    )],
                    vec![Material::new(rgba(1.0, 1.0, 1.0, 1.0), 0.5)],
                ),
            ),
        }
    }
}
impl Window for App {
    fn draw(&mut self, gui: &mut Gui) {
        gui.immediate_ui(|gui| {
            let ctx = &gui.egui_ctx;
            egui::CentralPanel::default().show(ctx, |ui| {
                self.viewer.show(ui);
            });
        });
    }
}
