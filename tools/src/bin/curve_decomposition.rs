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
use space::hspace::HSpace2;
use space::EVector;
use spline::math::FloatRange;
use spline::nurbs_curve::NurbsCurve;
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
        let curve = NurbsCurve::<HSpace2>::example_circle();
        let beziers = curve.decompose();

        let num_segments = 50;
        let colors = [Rgba::BLUE, Rgba::RED, Rgba::GREEN, Rgba::CYAN];
        let mut bezier_edges: Vec<ModelEdge> = Vec::new();
        for (i, bezier) in beziers.iter().enumerate() {
            bezier_edges.push(ModelEdge::new(
                0.into(),
                FloatRange::new(0.0, 1.0, num_segments)
                    .map(|u| EdgeVertex {
                        position: bezier.point(u).f32s(),
                        expand: [0.0, 0.0, 0.0],
                    })
                    .collect::<Vec<_>>(),
                colors[i],
            ));
        }

        let num_segments = 50 * beziers.len();
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
                .edges(bezier_edges)
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
