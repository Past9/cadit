use components::{rgba, Rgba};
use render::{
    mesh::{Edge, EdgeVertex},
    model::ModelEdge,
};

pub fn make_grid(size: usize, xy: bool, xz: bool, yz: bool) -> Vec<ModelEdge> {
    let gs = size as isize;
    let color = rgba(0.0, 0.075, 0.15, 1.0);

    let xy_lines = if xy {
        (-gs..=gs)
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
                    match x == 0 {
                        true => Rgba::DARK_GREEN,
                        false => color,
                    },
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
                    match y == 0 {
                        true => Rgba::DARK_RED,
                        false => color,
                    },
                )
            }))
            .collect()
    } else {
        Vec::new()
    };

    let xz_lines = if xz {
        (-gs..=gs)
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
                    match x == 0 {
                        true => Rgba::DARK_BLUE,
                        false => color,
                    },
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
                    match z == 0 {
                        true => Rgba::DARK_RED,
                        false => color,
                    },
                )
            }))
            .collect()
    } else {
        Vec::new()
    };

    let yz_lines = if yz {
        (-gs..=gs)
            .map(|y| {
                ModelEdge::new(
                    0.into(),
                    Edge {
                        vertices: vec![
                            EdgeVertex {
                                position: [0.0, y as f32, gs as f32],
                                expand: [0.0, 0.0, 0.0],
                            },
                            EdgeVertex {
                                position: [0.0, y as f32, -gs as f32],
                                expand: [0.0, 0.0, 0.0],
                            },
                        ],
                    },
                    match y == 0 {
                        true => Rgba::DARK_BLUE,
                        false => color,
                    },
                )
            })
            .chain((-gs..=gs).map(|z| {
                ModelEdge::new(
                    0.into(),
                    Edge {
                        vertices: vec![
                            EdgeVertex {
                                position: [0.0, gs as f32, z as f32],
                                expand: [0.0, 0.0, 0.0],
                            },
                            EdgeVertex {
                                position: [0.0, -gs as f32, z as f32],
                                expand: [0.0, 0.0, 0.0],
                            },
                        ],
                    },
                    match z == 0 {
                        true => Rgba::DARK_GREEN,
                        false => color,
                    },
                )
            }))
            .collect()
    } else {
        Vec::new()
    };

    xy_lines
        .into_iter()
        .chain(xz_lines.into_iter())
        .chain(yz_lines.into_iter())
        .collect()
}
