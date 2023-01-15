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
use spline::math::{FloatRange, Vec3H};
use components::{rgba, scene::SceneViewer, Gui};
use components::{run_window, Window, WindowDescriptor};

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
        let gs = 50;
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

        // Create curve
        let curve = spline::curve::Curve::<Vec3H>::example_quarter_circle();

        let closest = curve.closest(spline::math::Vec3::new(-1.0, 0.0, 0.0), 0.0);

        println!("CLOSEST {}", closest);

        let num_segments = 50;
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
            point3(-0.25, -0.5, -1.0),
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
                        vec![curve_edge],
                        //curve_points.iter().map(|p| p.1.clone()).collect(),
                        grid_points.into_iter().collect(),
                    )],
                    vec![Material::new(rgba(1.0, 1.0, 1.0, 1.0), 0.5)],
                ),
            ),
        }

        /*
        let start = Instant::now();

        let curve = spline::curves::nurbs::NurbsCurve::example_simple();

        let test_point = Point3::new(0.0, -2.0, 0.0);
        //let projected_u = curve.project_point(test_point, 0.4);
        //println!("Projected U: {}", projected_u);

        //let curve = curve.derivative_curve();

        let points = curve
            .control_points
            .iter()
            .map(|p| {
                ModelPoint::new(
                    0.into(),
                    Point {
                        position: [p.x as f32, p.y as f32, p.z as f32],
                        expand: [0.0, 0.0, 0.0],
                    },
                )
            })
            .collect::<Vec<_>>();

        let end = Instant::now();

        println!("Constructed in {}ms", (end - start).as_millis());

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
                    vec![Model::new(vec![], vec![], points)],
                    vec![Material::new(rgba(1.0, 1.0, 1.0, 1.0), 0.5)],
                ),
            ),
        }
        */
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
