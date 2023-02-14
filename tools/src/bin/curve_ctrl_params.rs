use cgmath::{point3, vec3, Deg, InnerSpace};
use components::{rgba, scene::SceneViewer, Gui};
use components::{run_window, Window, WindowDescriptor};
use eframe::egui;
use render::model::Geometry;
use render::scene::SceneBuilder;
use render::{
    camera::{Camera, CameraAngle},
    model::EdgeVertex,
    model::{Model, ModelEdge},
    Rgba,
};
use space::hspace::{HSpace, HSpace2};
use space::{EVector, HVec2};
use spline::math::FloatRange;
use spline::{math::knot_vector::KnotVector, nurbs_curve::NurbsCurve};
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
            closest_lines.push(ModelEdge::new(
                0.into(),
                vec![
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
                Rgba::CYAN,
            ));
        }

        let num_segments = 200;
        let curve_edge = ModelEdge::new(
            0.into(),
            FloatRange::new(curve.min_u(), curve.max_u(), num_segments)
                .map(|u| EdgeVertex {
                    position: curve.point(u).f32s(),
                    expand: [0.0, 0.0, 0.0],
                })
                .collect::<Vec<_>>(),
            Rgba::YELLOW,
        );

        let mut geometry = Geometry::new();
        geometry.insert_model(
            Model::empty()
                .edges(closest_lines)
                .edge(curve_edge)
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
