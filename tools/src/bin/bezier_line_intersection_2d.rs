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
use spline::bezier_curve::{BezierCurve, Line2D};
use spline::math::knot_vector::KnotVector;
use spline::math::{FloatRange, Vec2, Vec2H, Vec3H};
use spline::nurbs_curve::NurbsCurve;

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
        let gs = 5;
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

        let original_bezier = BezierCurve::new(vec![
            Vec2H::new(-4.0, -4.0, 1.0),
            Vec2H::new(-2.0, 4.0, 1.0),
            Vec2H::new(2.0, -4.0, 1.0),
            Vec2H::new(4.0, 4.0, 1.0),
        ]);

        let num_segments = 50;
        let original_bezier_edge = ModelEdge::new(
            0.into(),
            Edge {
                vertices: FloatRange::new(0.0, 1.0, num_segments)
                    .map(|u| {
                        //
                        let floats = original_bezier.point(u).f32s();
                        EdgeVertex {
                            position: [floats[0], floats[1], 0.0],
                            expand: [0.0, 0.0, 0.0],
                        }
                    })
                    .collect::<Vec<_>>(),
            },
            Rgba::YELLOW,
        );

        let line = Line2D::from_pos_and_dir(Vec2::new(0.0, 0.0), Vec2::new(-1.5, -1.0));

        println!("LINE {:?}", line);
        //let line = Line2D::new(-1.0, 1.1, 0.0);

        let bezier = original_bezier.line_intersection_curve(line);

        let num_segments = 50;
        let bezier_edge = ModelEdge::new(
            0.into(),
            Edge {
                vertices: FloatRange::new(0.0, 1.0, num_segments)
                    .map(|u| {
                        //
                        let floats = bezier.point(u).f32s();
                        EdgeVertex {
                            position: [floats[0], floats[1], 0.0],
                            expand: [0.0, 0.0, 0.0],
                        }
                    })
                    .collect::<Vec<_>>(),
            },
            Rgba::BLUE,
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
                        point3(0.0, 0.0, -5.0),
                        vec3(0.0, 0.0, 1.0),
                        vec3(0.0, -1.0, 0.0).normalize(),
                        Deg(70.0).into(),
                        0.01,
                        5.0,
                    ),
                    vec![Model::new(
                        vec![],
                        vec![original_bezier_edge, bezier_edge],
                        grid_points.into_iter().collect(),
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
