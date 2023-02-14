use std::time::Instant;

use cgmath::{point3, vec3, Deg, InnerSpace};
use components::{rgba, scene::SceneViewer, Gui};
use components::{run_window, Window, WindowDescriptor};
use eframe::egui;
use render::model::Geometry;
use render::scene::SceneBuilder;
use render::{
    camera::{Camera, CameraAngle},
    model::EdgeVertex,
    model::{Model, ModelEdge, ModelPoint},
    Rgba,
};
use space::hspace::{HSpace, HSpace3};
use space::{EVector, HVec3};
use spline::bezier_curve::BezierCurve;
use spline::math::FloatRange;
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
        const SHOW_INTERSECTION_PLOT: bool = false;
        const SHOW_HAUSDORFF_PLOT: bool = false;

        let (curve, curve_edge) = {
            let curve = BezierCurve::<HSpace3>::new(vec![
                HVec3::new(-4.1, -4.0, -4.0, 1.0),
                HVec3::new(-7.0, 3.0, -12.0, 20.0),
                HVec3::new(-3.0, 5.0, -8.0, 10.0),
                HVec3::new(2.0, 5.0, 4.0, 20.0),
                HVec3::new(6.0, 1.0, 12.0, 1.0),
                HVec3::new(5.0, -5.0, 8.0, 30.0),
                HVec3::new(-1.0, -8.0, -4.0, 20.0),
                HVec3::new(-5.0, -7.0, -9.0, 1.0),
                HVec3::new(-6.0, -2.0, -7.0, 20.0),
                HVec3::new(-3.0, 3.0, -8.0, 0.5),
                HVec3::new(1.0, 3.0, 10.0, 1.0),
                HVec3::new(0.1, 0.0, 0.1, 1.0),
            ]);

            let num_segments = 500;
            let curve_edge = ModelEdge::new(
                0.into(),
                FloatRange::new(0.0, 1.0, num_segments)
                    .map(|u| EdgeVertex {
                        position: curve.point(u).f32s(),
                        expand: [0.0, 0.0, 0.0],
                    })
                    .collect::<Vec<_>>(),
                Rgba::YELLOW,
            );

            (curve, curve_edge)
        };

        let start_u = 0.0;
        let end_u = 1.0;
        let (line, line_edge) = {
            let start_pt = curve.point(start_u);
            let end_pt = curve.point(end_u);

            let line = HSpace3::make_line(start_pt, start_pt - end_pt);

            let line_edge = ModelEdge::new(
                0.into(),
                vec![
                    EdgeVertex {
                        position: start_pt.f32s(),
                        expand: [0.0, 0.0, 0.0],
                    },
                    EdgeVertex {
                        position: end_pt.f32s(),
                        expand: [0.0, 0.0, 0.0],
                    },
                ],
                Rgba::MAGENTA,
            );

            (line, line_edge)
        };

        let intersection_points = {
            let points = curve.line_intersections(&line);

            let intersection_points = points
                .into_iter()
                .map(|p| {
                    //
                    ModelPoint::new(
                        0.into(),
                        point3(p.x as f32, p.y as f32, p.z as f32),
                        vec3(0.0, 0.0, 0.0),
                        Rgba::GREEN,
                    )
                })
                .collect::<Vec<_>>();

            intersection_points
        };

        let hausdorff_points = {
            let min_u = Some(start_u);
            let max_u = Some(end_u);
            let num_times = 1;
            let mut times = vec![0u128; num_times];
            for i in 0..num_times {
                let start = Instant::now();
                curve.hausdorff_to_line(&line, min_u, max_u, true);
                let dur = (Instant::now() - start).as_micros();
                times[i] = dur;
            }
            println!(
                "Mean Hausdorff time: {}μs",
                times.into_iter().sum::<u128>() as f64 / num_times as f64
            );

            let hausdorff = curve.hausdorff_to_line(&line, min_u, max_u, true);

            if let Some(ref hausdorff) = hausdorff {
                println!("Hausdorff distance: {}", hausdorff.distance);
                println!("Hausdorff point: {:?}", hausdorff.point);
                println!("Hausdorff U: {}", hausdorff.u);
            }

            let points = curve.hausdorff_candidates(&line, min_u, max_u, true);
            let hausdorff_points = points
                .into_iter()
                .map(|p| {
                    let color = match &hausdorff {
                        Some(hausdorff) => {
                            if hausdorff.point == p.1 {
                                Rgba::WHITE
                            } else {
                                Rgba::RED
                            }
                        }
                        None => Rgba::RED,
                    };
                    ModelPoint::new(
                        0.into(),
                        point3(p.1.x as f32, p.1.y as f32, p.1.z as f32),
                        vec3(0.0, 0.0, 0.0),
                        color,
                    )
                })
                .collect::<Vec<_>>();

            hausdorff_points
        };

        let (intersection_self_edge, intersection_der_edge) = {
            let (self_plot, der_plot) = if SHOW_INTERSECTION_PLOT {
                curve.line_intersection_plot(&line, 200)
            } else {
                (vec![], vec![])
            };

            let self_edge = ModelEdge::new(
                0.into(),
                self_plot
                    .into_iter()
                    .map(|(u, pt)| EdgeVertex {
                        position: [u as f32, pt.x as f32 / 10.0, pt.y as f32 / 10.0],
                        expand: [0.0, 0.0, 0.0],
                    })
                    .collect(),
                Rgba::BLUE,
            );

            let der_edge = ModelEdge::new(
                0.into(),
                der_plot
                    .into_iter()
                    .map(|(u, pt)| EdgeVertex {
                        position: [u as f32, pt.x as f32 / 10.0, pt.y as f32 / 10.0],
                        expand: [0.0, 0.0, 0.0],
                    })
                    .collect(),
                Rgba::CYAN,
            );

            (self_edge, der_edge)
        };

        let (hausdorff_self_edge, hausdorff_der1_edge, hausdorff_der2_edge) = {
            let (self_plot, der1_plot, der2_plot) = if SHOW_HAUSDORFF_PLOT {
                curve.line_hausdorff_plot(&line, 1000)
            } else {
                (vec![], vec![], vec![])
            };

            let self_scale = 10.0;
            let der_scale = 50.0;

            let self_edge = ModelEdge::new(
                0.into(),
                self_plot
                    .into_iter()
                    .map(|(u, pt)| EdgeVertex {
                        position: [u as f32, pt.x as f32 / self_scale, pt.y as f32 / self_scale],
                        expand: [0.0, 0.0, 0.0],
                    })
                    .collect(),
                Rgba::BLUE,
            );

            let der1_edge = ModelEdge::new(
                0.into(),
                der1_plot
                    .into_iter()
                    .map(|(u, pt)| EdgeVertex {
                        position: [u as f32, pt.x as f32 / der_scale, pt.y as f32 / der_scale],
                        expand: [0.0, 0.0, 0.0],
                    })
                    .collect(),
                Rgba::CYAN,
            );

            let der2_edge = ModelEdge::new(
                0.into(),
                der2_plot
                    .into_iter()
                    .map(|(u, pt)| EdgeVertex {
                        position: [u as f32, pt.x as f32 / der_scale, pt.y as f32 / der_scale],
                        expand: [0.0, 0.0, 0.0],
                    })
                    .collect(),
                Rgba::GREEN,
            );

            (self_edge, der1_edge, der2_edge)
        };

        let mut geometry = Geometry::new();
        geometry.insert_model(
            Model::empty()
                .edge(curve_edge)
                .edge(line_edge)
                .edge(intersection_self_edge)
                .edge(intersection_der_edge)
                .edge(hausdorff_self_edge)
                .edge(hausdorff_der1_edge)
                .edge(hausdorff_der2_edge)
                .edges(make_grid(15, true, true, true))
                .points(intersection_points)
                .points(hausdorff_points),
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

impl Default for App {
    fn default() -> Self {
        Self::new()
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
