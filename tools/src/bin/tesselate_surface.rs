use cgmath::{point3, vec3, Deg, InnerSpace, Vector3, Vector4};
use components::{rgb, run_window, Window, WindowDescriptor};
use components::{rgba, scene::SceneViewer, Gui};
use eframe::egui;
use render::lights::{AmbientLight, Lights};
use render::model::{Geometry, TranslucentMaterial};
use render::scene::SceneBuilder;
use render::{
    camera::{Camera, CameraAngle},
    lights::DirectionalLight,
    model::{Model, OpaqueMaterial},
    scene::Scene,
    Rgb,
};
use space::hspace::HSpace3;
use space::HVec3;
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

pub fn blend(bg: Vector3<f32>, colors: Vec<Vector4<f32>>) -> Vector3<f32> {
    let mut output = bg;

    for color in colors.iter() {
        let a = color.w;
        let rgb = vec3(color.x, color.y, color.z);

        output.x *= 1.0 - a + a * rgb.x;
        output.y *= 1.0 - a + a * rgb.y;
        output.z *= 1.0 - a + a * rgb.z;
    }

    output
}

pub struct App {
    viewer: SceneViewer,
}
impl App {
    pub fn new() -> Self {
        let mut surface1 = spline::bezier_surface::BezierSurface::<HSpace3>::example_simple();
        surface1.translate(HVec3::new(0.0, 1.0, 0.0, 0.0));

        let mut surface2 = spline::bezier_surface::BezierSurface::<HSpace3>::example_simple();
        surface2.translate(HVec3::new(0.0, -1.0, 0.0, 0.0));

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
                .surface(naive::tesselate_bezier_surface(
                    &surface2,
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
