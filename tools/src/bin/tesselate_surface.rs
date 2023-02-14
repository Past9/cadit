use cgmath::{point3, vec3, Deg, InnerSpace, Vector3, Vector4};
use components::{rgb, run_window, Rgba, Window, WindowDescriptor};
use components::{rgba, scene::SceneViewer, Gui};
use eframe::egui;
use render::lights::Lights;
use render::model::{Geometry, ModelPoint};
use render::scene::SceneBuilder;
use render::{
    camera::{Camera, CameraAngle},
    model::Model,
    Rgb,
};
use space::hspace::HSpace3;
use space::{EVec3, EVector, HVec3};
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
        surface1.translate(HVec3::new(0.0, 0.0, 0.0, 0.0));

        let plane =
            space::EPlane3::new_from_normal_vec(EVec3::new(1.0, -1.0, 0.0).normalize(), 0.0);
        let hausdorff_candidates = surface1.hausdorff_candidates(&plane, None, None);

        println!("{} {:?}", hausdorff_candidates.len(), hausdorff_candidates);

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
                .points(
                    hausdorff_candidates
                        .into_iter()
                        .map(|c| {
                            ModelPoint::new(
                                0.into(),
                                point3(c.1.x as f32, c.1.y as f32, c.1.z as f32),
                                vec3(0.0, 0.0, 0.0),
                                Rgba::WHITE,
                            )
                        })
                        .collect(),
                )
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
