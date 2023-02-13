use cgmath::{point3, vec3, Deg, InnerSpace};
use components::{rgb, run_window, Window, WindowDescriptor};
use components::{rgba, scene::SceneViewer, Gui};
use eframe::egui;
use render::{
    camera::{Camera, CameraAngle},
    lights::DirectionalLight,
    model::{Edge, EdgeVertex, Point},
    model::{Model, ModelEdge, ModelPoint, OpaqueMaterial},
    scene::{Scene, SceneLights},
    Rgb, Rgba,
};
use space::hspace::{HSpace, HSpace2};
use space::{EVector, HVec2};
use spline::math::FloatRange;
use spline::{math::knot_vector::KnotVector, nurbs_curve::NurbsCurve};

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
        let gs = 5;
        let grid_points = (-gs..=gs)
            .flat_map(|x| {
                (-gs..=gs).map(move |y| {
                    let mut color = rgba(0.0, 0.05, 0.15, 1.0);
                    if x == 0 && y == 0 {
                        color = Rgba::RED;
                    }
                    ModelPoint::new(
                        0.into(),
                        Point {
                            position: [x as f32, y as f32, 0.0],
                            expand: [0.0, 0.0, 0.0],
                        },
                        color,
                    )
                })
            })
            .collect::<Vec<_>>();

        let curve = NurbsCurve::<HSpace2>::new(
            Vec::from([
                HVec2::new(-3.0, 0.0, 1.0),
                HVec2::new(-2.0, -4.0, 1.0),
                HVec2::new(0.0, 8.0, 1.0),
                HVec2::new(2.0, -4.0, 1.0),
                HVec2::new(3.0, 0.0, 1.0),
            ]),
            KnotVector::new([0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0]),
        );

        let mut closest_lines: Vec<ModelEdge> = Vec::new();
        for (i, point) in curve.control_points().iter().enumerate() {
            let u = curve.u_at_control_point(i);
            let closest = curve
                .find_closest(HSpace2::project_vec(point.clone()), u, 20)
                .unwrap();
            println!("{:?} {:#?}", HSpace2::project_vec(point.clone()), closest);
            closest_lines.push(ModelEdge::new(
                0.into(),
                Edge {
                    vertices: vec![
                        EdgeVertex {
                            position: [point.x as f32, point.y as f32, 0.0],
                            expand: [0.0, 0.0, 0.0],
                        },
                        EdgeVertex {
                            position: [
                                closest.closest_point.x as f32,
                                closest.closest_point.y as f32,
                                0.0,
                            ],
                            expand: [0.0, 0.0, 0.0],
                        },
                    ],
                },
                Rgba::CYAN,
            ));
        }

        let num_segments = 200;
        let curve_edge = ModelEdge::new(
            0.into(),
            Edge {
                vertices: FloatRange::new(curve.min_u(), curve.max_u(), num_segments)
                    .map(|u| EdgeVertex {
                        position: curve.point(u).f32s(),
                        expand: [0.0, 0.0, 0.0],
                    })
                    .collect::<Vec<_>>(),
            },
            Rgba::YELLOW,
        );

        let cam = Camera::create_perspective(
            [0, 0],
            point3(0.0, 0.0, -3.0),
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, -1.0, 0.0).normalize(),
            Deg(70.0).into(),
            0.01,
            5.0,
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
                    cam,
                    vec![Model::new(
                        vec![],
                        vec![],
                        [curve_edge]
                            .into_iter()
                            .chain(closest_lines.into_iter())
                            .collect(),
                        grid_points.into_iter().collect(),
                    )],
                    vec![OpaqueMaterial::new(rgb(1.0, 1.0, 1.0), 0.5)],
                    vec![],
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
