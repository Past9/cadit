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
use spline::math::FloatRange;
use spline::nurbs_curve::NurbsCurve;
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
        println!(
            r#"
Edge is displayed in yellow, normals in green, and reverse normals in red. If normals are correct,
the red lines should stop just short of the origin and should form a perfect circle around it. This
demo indicates that curve normals, tangents, and first derivatives are correctly calculated.
"#
        );

        let curve = NurbsCurve::<HSpace2>::example_circle();

        let num_segments = 360;
        let mut curve_edge_vertices: Vec<EdgeVertex> = Vec::new();
        let mut normal_edges: Vec<ModelEdge> = Vec::new();
        let mut rev_normal_edges: Vec<ModelEdge> = Vec::new();

        for u in FloatRange::new(curve.min_u(), curve.max_u(), num_segments) {
            let point = curve.point(u);
            curve_edge_vertices.push(EdgeVertex {
                position: [point.x as f32, point.y as f32, 0.0],
                expand: [0.0, 0.0, 0.0],
            });

            let normal = curve.normal(u);
            let normal_end = point + normal;

            normal_edges.push(ModelEdge::new(
                0.into(),
                vec![
                    EdgeVertex {
                        position: [point.x as f32, point.y as f32, 0.0],
                        expand: [0.0, 0.0, 0.0],
                    },
                    EdgeVertex {
                        position: [normal_end.x as f32, normal_end.y as f32, 0.0],
                        expand: [0.0, 0.0, 0.0],
                    },
                ],
                Rgba::GREEN,
            ));

            let rev_normal_end = point - normal * 0.99;
            rev_normal_edges.push(ModelEdge::new(
                0.into(),
                vec![
                    EdgeVertex {
                        position: [point.x as f32, point.y as f32, 0.0],
                        expand: [0.0, 0.0, 0.0],
                    },
                    EdgeVertex {
                        position: [rev_normal_end.x as f32, rev_normal_end.y as f32, 0.0],
                        expand: [0.0, 0.0, 0.0],
                    },
                ],
                Rgba::RED,
            ));
        }

        let curve_edge = ModelEdge::new(0.into(), curve_edge_vertices, Rgba::YELLOW);

        let mut geometry = Geometry::new();
        geometry.insert_model(
            Model::empty()
                .edges(normal_edges)
                .edges(rev_normal_edges)
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
