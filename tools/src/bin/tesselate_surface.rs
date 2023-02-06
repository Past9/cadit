use cgmath::{point3, vec3, vec4, Deg, InnerSpace, Vector3, Vector4, VectorSpace};
use components::{rgb, run_window, Window, WindowDescriptor};
use components::{rgba, scene::SceneViewer, Gui};
use eframe::egui;
use render::model::TranslucentMaterial;
use render::{
    camera::{Camera, CameraAngle},
    lights::DirectionalLight,
    model::{Model, OpaqueMaterial},
    scene::{Scene, SceneLights},
    Rgb, Rgba,
};
use space::hspace::HSpace3;
use space::HVec3;
use std::time::Instant;
use tesselate::exact::tesselate_bezier_curve;
use tesselate::naive::{self, tesselate_bezier_surface};
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
        /*
        let color = blend(
            vec3(1.0, 1.0, 1.0),
            vec![
                vec4(1.0, 0.0, 0.0, 1.0),
                //vec4(0.5, 1.0, 0.5, 0.5)
            ],
        );

        println!("{:?}", color);

        panic!();
        */

        let surface1 = spline::bezier_surface::BezierSurface::<HSpace3>::example_simple();

        let mut surface2 = spline::bezier_surface::BezierSurface::<HSpace3>::example_simple();
        surface2.translate(HVec3::new(0.0, 1.0, 0.0, 0.0));

        let mut surface3 = spline::bezier_surface::BezierSurface::<HSpace3>::example_simple();
        surface3.translate(HVec3::new(0.0, 3.0, 0.0, 0.0));

        let mut surface4 = spline::bezier_surface::BezierSurface::<HSpace3>::example_simple();
        surface4.translate(HVec3::new(0.0, -1.0, 0.0, 0.0));

        let model1 = tesselate_bezier_surface(&surface1, 20, 0.into(), 0);
        let model2 = tesselate_bezier_surface(&surface2, 20, 0.into(), 1);
        let model3 = tesselate_bezier_surface(&surface3, 20, 0.into(), 0);
        let model4 = tesselate_bezier_surface(&surface4, 20, 0.into(), 0);

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
                            DirectionalLight::new(
                                vec3(1.0, 0.0, 1.0).normalize(),
                                rgb(0.3, 0.3, 0.5),
                                1.0,
                            ),
                            DirectionalLight::new(
                                vec3(-1.0, 0.0, 1.0).normalize(),
                                rgb(0.5, 0.5, 0.3),
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
                        vec![model4, model3],
                        vec![model1, model2],
                        make_grid(10, true, true, true),
                        vec![],
                    )],
                    vec![OpaqueMaterial::new(rgb(0.8, 0.8, 0.8), 0.5)],
                    vec![
                        TranslucentMaterial::new(rgba(1.0, 0.0, 0.0, 1.0), 0.5),
                        TranslucentMaterial::new(rgba(0.0, 1.0, 0.0, 1.0), 0.5),
                    ],
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
