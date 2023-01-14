use cgmath::{point3, vec3, Deg, InnerSpace};
use eframe::egui;
use render::{
    camera::{Camera, CameraAngle},
    lights::DirectionalLight,
    mesh::{Edge, EdgeVertex, Point},
    model::{Material, Model, ModelEdge, ModelPoint},
    scene::{Scene, SceneLights},
    Rgb, Rgba,
};
use spline::math::{FloatRange, Vec2H, Vec3H};
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
        println!(
            r#"
Edge is displayed in yellow, normals in green, and reverse normals in red. If normals are correct,
the red lines should stop just short of the origin and should form a perfect circle around it. This
demo indicates that curve normals, tangents, and first derivatives are working correctly.
"#
        );

        let curve = spline::curve::Curve::<Vec2H>::example_circle();

        let gs = 3;
        let grid_points = (-gs..=gs)
            .flat_map(|x| {
                (-gs..=gs).map(move |y| {
                    ModelPoint::new(
                        0.into(),
                        Point {
                            position: [x as f32, y as f32, 0.0],
                            expand: [0.0, 0.0, 0.0],
                        },
                        rgba(0.0, 0.05, 0.15, 1.0),
                    )
                })
            })
            .collect::<Vec<_>>();

        let num_segments = 360;
        let mut curve_edge_vertices: Vec<EdgeVertex> = Vec::new();
        let mut normal_edges: Vec<ModelEdge> = Vec::new();
        let mut rev_normal_edges: Vec<ModelEdge> = Vec::new();

        for u in FloatRange::new(curve.min_u(), curve.max_u(), num_segments) {
            let point = curve.point(u);
            curve_edge_vertices.push(EdgeVertex {
                position: [point.x as f32, point.y as f32, 0.0],
                expand: [0.0, 0.0, 0.0],
            });

            let normal = curve.normal(u);
            let normal_end = point + normal;

            normal_edges.push(ModelEdge::new(
                0.into(),
                Edge {
                    vertices: vec![
                        EdgeVertex {
                            position: [point.x as f32, point.y as f32, 0.0],
                            expand: [0.0, 0.0, 0.0],
                        },
                        EdgeVertex {
                            position: [normal_end.x as f32, normal_end.y as f32, 0.0],
                            expand: [0.0, 0.0, 0.0],
                        },
                    ],
                },
                Rgba::GREEN,
            ));

            let rev_normal_end = point - normal * 0.99;
            rev_normal_edges.push(ModelEdge::new(
                0.into(),
                Edge {
                    vertices: vec![
                        EdgeVertex {
                            position: [point.x as f32, point.y as f32, 0.0],
                            expand: [0.0, 0.0, 0.0],
                        },
                        EdgeVertex {
                            position: [rev_normal_end.x as f32, rev_normal_end.y as f32, 0.0],
                            expand: [0.0, 0.0, 0.0],
                        },
                    ],
                },
                Rgba::RED,
            ));
        }

        let curve_edge = ModelEdge::new(
            0.into(),
            Edge {
                vertices: curve_edge_vertices,
            },
            Rgba::YELLOW,
        );

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
                        point3(0.0, 0.0, -3.0),
                        vec3(0.0, 0.0, 1.0),
                        vec3(0.0, -1.0, 0.0).normalize(),
                        Deg(70.0).into(),
                        0.01,
                        5.0,
                    ),
                    vec![Model::new(
                        vec![],
                        [curve_edge]
                            .into_iter()
                            .chain(normal_edges.into_iter())
                            .chain(rev_normal_edges.into_iter())
                            .collect(),
                        grid_points.into_iter().collect(),
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
