use cgmath::{point3, vec3, Deg, InnerSpace};
use components::{rgba, scene::SceneViewer, Gui};
use components::{run_window, Window, WindowDescriptor};
use eframe::egui;
use render::model::Geometry;
use render::scene::SceneBuilder;
use render::{
    camera::{Camera, CameraAngle},
    model::Model,
    Rgba,
};
use space::hspace::HSpace3;
use std::time::Instant;
use tesselate::exact::tesselate_bezier_curve;
use tesselate::naive;
use tools::make_grid;

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
        let beziers = spline::nurbs_curve::NurbsCurve::<HSpace3>::example_crazy().decompose();

        let naive_edges = beziers
            .iter()
            .map(|b| naive::tesselate_bezier_curve(b, 5000, 0.into(), Rgba::BLACK))
            .collect::<Vec<_>>();

        let tolerance = 0.01;
        let start_time = Instant::now();
        let exact_edges = beziers
            .iter()
            .map(|bezier| {
                tesselate_bezier_curve(bezier, tolerance).to_model_edge(0.into(), Rgba::GREEN)
            })
            .collect::<Vec<_>>();
        println!(
            "Tesselated to {} in {}us",
            tolerance,
            (Instant::now() - start_time).as_micros()
        );

        let mut geometry = Geometry::new();
        geometry.insert_model(
            Model::empty()
                .edges(naive_edges)
                .edges(exact_edges)
                .edges(make_grid(5, true, true, true)),
        );

        let mut scene = SceneBuilder::empty();
        scene
            .background(rgba(0.05, 0.1, 0.15, 1.0))
            .camera(Camera::create_perspective(
                [0, 0],
                point3(0.0, 0.0, -1.0),
                vec3(0.0, 0.0, 1.0),
                vec3(0.0, -1.0, 0.0).normalize(),
                Deg(70.0).into(),
                0.01,
                5.0,
            ))
            .geometry(geometry);

        Self {
            viewer: SceneViewer::new(
                CameraAngle::Front.get_rotation(),
                vec3(0.0, 0.0, 0.0),
                true,
                true,
                true,
                scene.build(),
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
