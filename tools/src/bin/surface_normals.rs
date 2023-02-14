use cgmath::{point3, vec3, Deg, InnerSpace};
use components::{rgb, run_window, Rgb, Window, WindowDescriptor};
use components::{rgba, scene::SceneViewer, Gui};
use eframe::egui;
use render::lights::Lights;
use render::model::Geometry;
use render::scene::SceneBuilder;
use render::{
    camera::{Camera, CameraAngle},
    model::EdgeVertex,
    model::{Model, ModelEdge},
    Rgba,
};
use space::hspace::{HSpace2, HSpace3};
use space::{EVec3, HVec3};
use spline::math::FloatRange;
use spline::nurbs_curve::NurbsCurve;
use tesselate::naive;
use tools::make_grid;

pub fn main() {
    run_window(
        App::new(),
        &WindowDescriptor {
            position: Some([320.0, 50.0]),
            width: 1080.0,
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
        let mut surface1 =
            spline::bezier_surface::BezierSurface::<HSpace3>::example_eighth_sphere();

        let mut geometry = Geometry::new();
        let opaque_gray = geometry.insert_material(rgba(0.8, 0.8, 0.8, 1.0), 0.5);

        geometry.insert_model(
            Model::empty()
                .surface(naive::tesselate_bezier_surface(
                    &surface1,
                    20,
                    0.into(),
                    opaque_gray,
                ))
                .edges(make_grid(5, true, true, true)),
        );

        let mut scene = SceneBuilder::empty();
        scene
            .background(rgba(0.05, 0.1, 0.15, 1.0))
            .camera(Camera::create_perspective(
                [0, 0],
                point3(0.0, 0.0, -3.0),
                vec3(0.0, 0.0, 1.0),
                vec3(0.0, -1.0, 0.0).normalize(),
                Deg(70.0).into(),
                0.01,
                5.0,
            ))
            .lights(
                Lights::empty()
                    .ambient(Rgb::WHITE, 0.2)
                    .directional(vec3(1.0, 0.0, 1.0).normalize(), rgb(0.0, 0.0, 1.0), 0.3)
                    .directional(vec3(-1.0, 0.0, 1.0).normalize(), rgb(1.0, 1.0, 0.0), 0.3),
            )
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
