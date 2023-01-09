use std::time::Instant;

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
use spline::{
    math::{FloatRange, Homogeneous, Vec3},
    surfaces::nurbs::SurfaceDirection,
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
        let start = Instant::now();
        let surface = spline::surfaces::nurbs::NurbsSurface::example_1();
        let points = surface.points(100, 100);

        /*
        let curve = spline::curves::nurbs::NurbsCurve::example_simple();


        let res_u = 100;
        let res_v = 100;

        let points = curve.points(res_u);
        //let s1pts = points.clone();
        let s1pts = curve.control_points.iter().map(|p| p.clone());
        */

        let s1pts = points.iter().map(|p| p.iter()).flatten().map(|p| p.clone());
        /*
        let s1pts = surface
            .control_points
            .iter()
            .map(|p| p.iter())
            .flatten()
            .map(|p| p.clone());
            */

        //println!("{:#?}", surf1.control_points);

        /*
        let s2pts = surf2
            .control_points
            .iter()
            .map(|p| p.iter())
            .flatten()
            .map(|p| p.clone().cartesian_components() + Vec3::new(-offset, 0.0, offset));

        let s3pts = surf3
            .control_points
            .iter()
            .map(|p| p.iter())
            .flatten()
            .map(|p| p.clone().cartesian_components() + Vec3::new(offset, 0.0, offset));
        */

        let points = s1pts
            //.chain(s2pts)
            //.chain(s3pts)
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
