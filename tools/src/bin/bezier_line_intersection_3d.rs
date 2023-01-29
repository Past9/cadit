use std::time::Instant;

use cgmath::{point3, vec3, Deg, InnerSpace};
use components::{rgba, scene::SceneViewer, Gui};
use components::{run_window, Window, WindowDescriptor};
use eframe::egui;
use render::{
    camera::{Camera, CameraAngle},
    lights::DirectionalLight,
    mesh::{Edge, EdgeVertex, Point},
    model::{Material, Model, ModelEdge, ModelPoint},
    scene::{Scene, SceneLights},
    Rgb, Rgba,
};
use space::hspace::{HSpace, HSpace3};
use space::{ELine, ELine2, ELine3, EVec2, EVec3, EVector, HVec2, HVec3};
use spline::bezier_curve::BezierCurve;
use spline::math::FloatRange;

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

        let xy_grid_lines = {
            let gs = 5;
            let color = rgba(0.0, 0.075, 0.15, 1.0);
            let grid_lines = (-gs..=gs)
                .map(|x| {
                    ModelEdge::new(
                        0.into(),
                        Edge {
                            vertices: vec![
                                EdgeVertex {
                                    position: [x as f32, gs as f32, 0.0],
                                    expand: [0.0, 0.0, 0.0],
                                },
                                EdgeVertex {
                                    position: [x as f32, -gs as f32, 0.0],
                                    expand: [0.0, 0.0, 0.0],
                                },
                            ],
                        },
                        if x == 0 { Rgba::BLACK } else { color },
                    )
                })
                .chain((-gs..=gs).map(|y| {
                    ModelEdge::new(
                        0.into(),
                        Edge {
                            vertices: vec![
                                EdgeVertex {
                                    position: [gs as f32, y as f32, 0.0],
                                    expand: [0.0, 0.0, 0.0],
                                },
                                EdgeVertex {
                                    position: [-gs as f32, y as f32, 0.0],
                                    expand: [0.0, 0.0, 0.0],
                                },
                            ],
                        },
                        if y == 0 { Rgba::BLACK } else { color },
                    )
                }))
                .collect::<Vec<_>>();

            grid_lines
        };

        let xz_grid_lines = {
            let gs = 5;
            let color = rgba(0.0, 0.075, 0.15, 1.0);
            let grid_lines = (-gs..=gs)
                .map(|x| {
                    ModelEdge::new(
                        0.into(),
                        Edge {
                            vertices: vec![
                                EdgeVertex {
                                    position: [x as f32, 0.0, gs as f32],
                                    expand: [0.0, 0.0, 0.0],
                                },
                                EdgeVertex {
                                    position: [x as f32, 0.0, -gs as f32],
                                    expand: [0.0, 0.0, 0.0],
                                },
                            ],
                        },
                        if x == 0 { Rgba::BLACK } else { color },
                    )
                })
                .chain((-gs..=gs).map(|z| {
                    ModelEdge::new(
                        0.into(),
                        Edge {
                            vertices: vec![
                                EdgeVertex {
                                    position: [gs as f32, 0.0, z as f32],
                                    expand: [0.0, 0.0, 0.0],
                                },
                                EdgeVertex {
                                    position: [-gs as f32, 0.0, z as f32],
                                    expand: [0.0, 0.0, 0.0],
                                },
                            ],
                        },
                        if z == 0 { Rgba::BLACK } else { color },
                    )
                }))
                .collect::<Vec<_>>();

            grid_lines
        };

        let (curve, curve_edge) = {
            /*
            let curve = BezierCurve::new(vec![
                HVec3::new(-4.0, -1.0, -4.0, 1.0),
                HVec3::new(-2.0, 4.0, -2.0, 10.0),
                HVec3::new(2.0, -4.0, 2.0, 10.0),
                HVec3::new(4.0, 1.0, 4.0, 1.0),
            ]);
            */

            let curve = BezierCurve::<HSpace3>::new(vec![
                HVec3::new(-4.0, -1.0, 0.0, 1.0),
                HVec3::new(-2.0, 4.0, 2.0, 100.0),
                HVec3::new(2.0, -4.0, 0.0, 10.0),
                HVec3::new(4.0, 1.0, 0.0, 1.0),
            ]);

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

            //let curve = BezierCurve::<HSpace3>::example_quarter_circle_xy();

            let num_segments = 500;
            let curve_edge = ModelEdge::new(
                0.into(),
                Edge {
                    vertices: FloatRange::new(0.0, 1.0, num_segments)
                        .map(|u| EdgeVertex {
                            position: curve.point(u).f32s(),
                            expand: [0.0, 0.0, 0.0],
                        })
                        .collect::<Vec<_>>(),
                },
                Rgba::YELLOW,
            );

            (curve, curve_edge)
        };

        //let start_u = 1.0;
        //let end_u = 0.0;
        let start_u = 0.0;
        let end_u = 0.013175420102170307;
        let (line, line_edge) = {
            //let start = EVec3::new(-5.0, 3.5, 0.0);
            //let end = EVec3::new(5.0, -2.5, 0.0);
            let start_pt = curve.point(start_u);
            let end_pt = curve.point(end_u);

            let line = HSpace3::make_line(start_pt, start_pt - end_pt);

            let line_edge = ModelEdge::new(
                0.into(),
                Edge {
                    vertices: vec![
                        EdgeVertex {
                            position: start_pt.f32s(),
                            expand: [0.0, 0.0, 0.0],
                        },
                        EdgeVertex {
                            position: end_pt.f32s(),
                            expand: [0.0, 0.0, 0.0],
                        },
                    ],
                },
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
                        Point {
                            position: p.f32s(),
                            expand: [0.0, 0.0, 0.0],
                        },
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
                        Point {
                            position: p.1.f32s(),
                            expand: [0.0, 0.0, 0.0],
                        },
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
                Edge {
                    vertices: self_plot
                        .into_iter()
                        .map(|(u, pt)| EdgeVertex {
                            position: [u as f32, pt.x as f32 / 10.0, pt.y as f32 / 10.0],
                            expand: [0.0, 0.0, 0.0],
                        })
                        .collect(),
                },
                Rgba::BLUE,
            );

            let der_edge = ModelEdge::new(
                0.into(),
                Edge {
                    vertices: der_plot
                        .into_iter()
                        .map(|(u, pt)| EdgeVertex {
                            position: [u as f32, pt.x as f32 / 10.0, pt.y as f32 / 10.0],
                            expand: [0.0, 0.0, 0.0],
                        })
                        .collect(),
                },
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
                Edge {
                    vertices: self_plot
                        .into_iter()
                        .map(|(u, pt)| EdgeVertex {
                            position: [
                                u as f32,
                                pt.x as f32 / self_scale,
                                pt.y as f32 / self_scale,
                            ],
                            expand: [0.0, 0.0, 0.0],
                        })
                        .collect(),
                },
                Rgba::BLUE,
            );

            let der1_edge = ModelEdge::new(
                0.into(),
                Edge {
                    vertices: der1_plot
                        .into_iter()
                        .map(|(u, pt)| EdgeVertex {
                            position: [u as f32, pt.x as f32 / der_scale, pt.y as f32 / der_scale],
                            expand: [0.0, 0.0, 0.0],
                        })
                        .collect(),
                },
                Rgba::CYAN,
            );

            let der2_edge = ModelEdge::new(
                0.into(),
                Edge {
                    vertices: der2_plot
                        .into_iter()
                        .map(|(u, pt)| EdgeVertex {
                            position: [u as f32, pt.x as f32 / der_scale, pt.y as f32 / der_scale],
                            expand: [0.0, 0.0, 0.0],
                        })
                        .collect(),
                },
                Rgba::GREEN,
            );

            (self_edge, der1_edge, der2_edge)
        };

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
                        point3(0.0, 0.0, -6.0),
                        vec3(0.0, 0.0, 1.0),
                        vec3(0.0, -1.0, 0.0).normalize(),
                        Deg(70.0).into(),
                        0.01,
                        5.0,
                    ),
                    /*
                    Camera::create_orthographic(
                        [0, 0],
                        point3(0.0, 0.0, -20.0),
                        vec3(0.0, 0.0, 1.0),
                        vec3(0.0, -1.0, 0.0).normalize(),
                        15.0,
                        0.01,
                        100.0,
                    ),
                    */
                    vec![Model::new(
                        vec![],
                        vec![
                            curve_edge,
                            line_edge,
                            intersection_self_edge,
                            intersection_der_edge,
                            hausdorff_self_edge,
                            hausdorff_der1_edge,
                            hausdorff_der2_edge,
                        ]
                        .into_iter()
                        .chain(xy_grid_lines.into_iter())
                        .chain(xz_grid_lines.into_iter())
                        .collect(),
                        vec![]
                            .into_iter()
                            .chain(intersection_points.into_iter())
                            .chain(hausdorff_points.into_iter())
                            .collect(),
                    )],
                    vec![Material::new(rgba(1.0, 1.0, 1.0, 1.0), 0.5)],
                ),
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
