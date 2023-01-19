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
use spline::bezier_curve::BezierCurve;
use spline::math::knot_vector::KnotVector;
use spline::math::line::{Line, Line2};
use spline::math::{FloatRange, Vec2, Vec2H, Vec3H};
use spline::nurbs_curve::NurbsCurve;

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
        let grid_lines = {
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
                        color,
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
                        color,
                    )
                }))
                .collect::<Vec<_>>();

            grid_lines
        };

        let (curve, curve_edge) = {
            let curve = BezierCurve::new(vec![
                Vec2H::new(-4.0, -1.0, 1.0),
                Vec2H::new(-2.0, 4.0, 10.0),
                Vec2H::new(2.0, -4.0, 10.0),
                Vec2H::new(4.0, 1.0, 1.0),
            ]);

            /*
            let curve = BezierCurve::new(vec![
                Vec2H::new(-4.1, -4.0, 1.0),
                Vec2H::new(-7.0, 3.0, 20.0),
                Vec2H::new(-3.0, 5.0, 10.0),
                Vec2H::new(2.0, 5.0, 20.0),
                Vec2H::new(6.0, 1.0, 1.0),
                Vec2H::new(5.0, -5.0, 30.0),
                Vec2H::new(-1.0, -8.0, 20.0),
                Vec2H::new(-5.0, -7.0, 1.0),
                Vec2H::new(-6.0, -2.0, 20.0),
                Vec2H::new(-3.0, 3.0, 0.5),
                Vec2H::new(1.0, 3.0, 1.0),
                Vec2H::new(0.1, 0.0, 1.0),
            ]);
            */

            /*
            let nurbs = NurbsCurve::new(
                vec![
                    Vec2H::new(-4.0, -1.0, 1.0),
                    Vec2H::new(-2.0, 4.0, 10.0),
                    Vec2H::new(2.0, -4.0, 10.0),
                    Vec2H::new(4.0, 1.0, 1.0),
                ],
                KnotVector::new([0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0]),
            )
            .derivative_curve(1);

            println!("NURBS DER {:#?}", nurbs.derivative_curve(1));
            */

            //let curve = BezierCurve::example_quarter_circle();

            let num_segments = 500;
            let curve_edge = ModelEdge::new(
                0.into(),
                Edge {
                    vertices: FloatRange::new(0.0, 1.0, num_segments)
                        .map(|u| {
                            //
                            let floats = curve.point(u).f32s();
                            EdgeVertex {
                                position: [floats[0], floats[1], 0.0],
                                expand: [0.0, 0.0, 0.0],
                            }
                        })
                        .collect::<Vec<_>>(),
                },
                Rgba::YELLOW,
            );

            (curve, curve_edge)
        };

        let (line, line_edge) = {
            //let start = Vec2::new(-5.0, 0.0);
            //let end = Vec2::new(5.0, 5.0);

            //let start = Vec2::new(10.0, -5.0);
            //let end = Vec2::new(10.0, 5.0);

            //let start = Vec2::new(-0.85, -5.0);
            //let end = Vec2::new(-0.15, 5.0);

            let start = Vec2::new(-5.0, 3.5);
            let end = Vec2::new(5.0, -2.5);

            let line = Line2::from_pos_and_dir(start, start - end);

            let start_f32s = start.f32s();
            let end_f32s = end.f32s();

            let line_edge = ModelEdge::new(
                0.into(),
                Edge {
                    vertices: vec![
                        EdgeVertex {
                            position: [start_f32s[0], start_f32s[1], 0.0],
                            expand: [0.0, 0.0, 0.0],
                        },
                        EdgeVertex {
                            position: [end_f32s[0], end_f32s[1], 0.0],
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
                    let floats = p.f32s();
                    ModelPoint::new(
                        0.into(),
                        Point {
                            position: [floats[0], floats[1], 0.0],
                            expand: [0.0, 0.0, 0.0],
                        },
                        Rgba::GREEN,
                    )
                })
                .collect::<Vec<_>>();

            intersection_points
        };

        let deviation_points = {
            let start = Instant::now();

            let hausdorff = curve.line_hausdorff(&line);

            println!("Hausdorff took {}μs", (Instant::now() - start).as_micros());
            println!("Hausdorff distance: {}", hausdorff.distance);

            if let Some(point) = hausdorff.point {
                println!("Hausdorff point: {:?}", point);
            }

            if let Some(u) = hausdorff.u {
                println!("Hausdorff U: {}", u);
            }

            let points = curve.line_hausdorff_candidates(&line);
            let deviation_points = points
                .into_iter()
                .map(|p| {
                    //
                    let floats = p.1.f32s();
                    ModelPoint::new(
                        0.into(),
                        Point {
                            position: [floats[0], floats[1], 0.0],
                            expand: [0.0, 0.0, 0.0],
                        },
                        Rgba::RED,
                    )
                })
                .collect::<Vec<_>>();

            deviation_points
        };

        let intersection_plot = {
            let points = curve.line_intersection_plot(&line);
            let intersection_plot = ModelEdge::new(
                0.into(),
                Edge {
                    vertices: points
                        .into_iter()
                        .map(|pt| {
                            //
                            let floats = pt.f32s();
                            EdgeVertex {
                                position: [floats[0], floats[1], 0.0],
                                expand: [0.0, 0.0, 0.0],
                            }
                        })
                        .collect::<Vec<_>>(),
                },
                Rgba::BLUE,
            );

            intersection_plot
        };

        let der_plot = {
            let points = curve.line_derivative_plot(&line);
            let der_plot = ModelEdge::new(
                0.into(),
                Edge {
                    vertices: points
                        .into_iter()
                        .map(|pt| {
                            //
                            let floats = pt.f32s();
                            EdgeVertex {
                                position: [floats[0], floats[1], 0.0],
                                expand: [0.0, 0.0, 0.0],
                            }
                        })
                        .collect::<Vec<_>>(),
                },
                Rgba::CYAN,
            );

            der_plot
        };

        /*
        let der_intersection_plot = {
            let points = curve.der1_curve_plot(&line);
            let der_intersection_plot = ModelEdge::new(
                0.into(),
                Edge {
                    vertices: points
                        .into_iter()
                        .map(|pt| {
                            //
                            let floats = pt.f32s();
                            EdgeVertex {
                                position: [floats[0], floats[1], 0.0],
                                expand: [0.0, 0.0, 0.0],
                            }
                        })
                        .collect::<Vec<_>>(),
                },
                Rgba::CYAN,
            );

            der_intersection_plot
        };
        */

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
                        //vec3(6.0, 10.0, 0.0).normalize(),
                        vec3(0.0, -1.0, 0.0).normalize(),
                        Deg(70.0).into(),
                        0.01,
                        5.0,
                    ),
                    vec![Model::new(
                        vec![],
                        vec![
                            curve_edge,
                            intersection_plot,
                            der_plot,
                            //der_intersection_plot,
                            line_edge,
                        ]
                        .into_iter()
                        .chain(grid_lines.into_iter())
                        .collect(),
                        vec![]
                            .into_iter()
                            .chain(intersection_points.into_iter())
                            .chain(deviation_points.into_iter())
                            .collect(),
                    )],
                    vec![Material::new(rgba(1.0, 1.0, 1.0, 1.0), 0.5)],
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
