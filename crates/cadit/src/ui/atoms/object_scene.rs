use eframe::{
    egui::{self, PointerButton},
    epaint::{mutex::Mutex, Pos2},
};
use std::sync::Arc;
use three_d::{Deg, Quaternion, Rotation3};

use crate::{
    render::{
        color_id::{ColorId, ColorIdSource},
        scene::Scene,
    },
    ui::{math::AnimatedValue, GlowContext},
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum CameraAngle {
    // Faces
    Front,
    Right,
    Back,
    Left,
    Top,
    Bottom,

    // Edges
    FrontRight,
    BackRight,
    BackLeft,
    FrontLeft,
    FrontTop,
    BackTop,
    FrontBottom,
    BackBottom,
    RightTop,
    LeftTop,
    RightBottom,
    LeftBottom,

    // Corners
    FrontRightTop,
    BackRightTop,
    BackLeftTop,
    FrontLeftTop,
    FrontRightBottom,
    BackRightBottom,
    BackLeftBottom,
    FrontLeftBottom,
}
impl CameraAngle {
    pub fn from_name(name: &str) -> Option<Self> {
        match &*name {
            "Front" => Some(Self::Front),
            "Right" => Some(Self::Right),
            "Back" => Some(Self::Back),
            "Left" => Some(Self::Left),
            "Top" => Some(Self::Top),
            "Bottom" => Some(Self::Bottom),
            "FrontRight" => Some(Self::FrontRight),
            "BackRight" => Some(Self::BackRight),
            "BackLeft" => Some(Self::BackLeft),
            "FrontLeft" => Some(Self::FrontLeft),
            "FrontTop" => Some(Self::FrontTop),
            "BackTop" => Some(Self::BackTop),
            "FrontBottom" => Some(Self::FrontBottom),
            "BackBottom" => Some(Self::BackBottom),
            "RightTop" => Some(Self::RightTop),
            "LeftTop" => Some(Self::LeftTop),
            "RightBottom" => Some(Self::RightBottom),
            "LeftBottom" => Some(Self::LeftBottom),
            "FrontRightTop" => Some(Self::FrontRightTop),
            "BackRightTop" => Some(Self::BackRightTop),
            "BackLeftTop" => Some(Self::BackLeftTop),
            "FrontLeftTop" => Some(Self::FrontLeftTop),
            "FrontRightBottom" => Some(Self::FrontRightBottom),
            "BackRightBottom" => Some(Self::BackRightBottom),
            "BackLeftBottom" => Some(Self::BackLeftBottom),
            "FrontLeftBottom" => Some(Self::FrontLeftBottom),
            _ => None,
        }
    }

    pub fn get_rotation(&self) -> Quaternion<f32> {
        let x: i32 = match self {
            // Faces
            CameraAngle::Front => 0,
            CameraAngle::Right => 0,
            CameraAngle::Back => 0,
            CameraAngle::Left => 0,
            CameraAngle::Top => 270,
            CameraAngle::Bottom => 90,

            // Edges
            CameraAngle::FrontRight => 0,
            CameraAngle::BackRight => 0,
            CameraAngle::BackLeft => 0,
            CameraAngle::FrontLeft => 0,
            CameraAngle::FrontTop => -45,
            CameraAngle::BackTop => -45,
            CameraAngle::FrontBottom => 45,
            CameraAngle::BackBottom => 45,
            CameraAngle::RightTop => -45,
            CameraAngle::LeftTop => -45,
            CameraAngle::RightBottom => 45,
            CameraAngle::LeftBottom => 45,

            // Corners
            CameraAngle::FrontRightTop => -45,
            CameraAngle::BackRightTop => -45,
            CameraAngle::BackLeftTop => -45,
            CameraAngle::FrontLeftTop => -45,
            CameraAngle::FrontRightBottom => 45,
            CameraAngle::BackRightBottom => 45,
            CameraAngle::BackLeftBottom => 45,
            CameraAngle::FrontLeftBottom => 45,
        };

        let y: i32 = match self {
            // Faces
            CameraAngle::Front => 0,
            CameraAngle::Right => -90,
            CameraAngle::Back => 180,
            CameraAngle::Left => 90,
            CameraAngle::Top => 0,
            CameraAngle::Bottom => 0,

            // Edges
            CameraAngle::FrontRight => -45,
            CameraAngle::BackRight => -135,
            CameraAngle::BackLeft => 135,
            CameraAngle::FrontLeft => 45,
            CameraAngle::FrontTop => 0,
            CameraAngle::BackTop => 180,
            CameraAngle::FrontBottom => 0,
            CameraAngle::BackBottom => 180,
            CameraAngle::RightTop => -90,
            CameraAngle::LeftTop => 90,
            CameraAngle::RightBottom => -90,
            CameraAngle::LeftBottom => 90,

            // Corners
            CameraAngle::FrontRightTop => -45,
            CameraAngle::BackRightTop => -135,
            CameraAngle::BackLeftTop => 135,
            CameraAngle::FrontLeftTop => 45,
            CameraAngle::FrontRightBottom => -45,
            CameraAngle::BackRightBottom => -135,
            CameraAngle::BackLeftBottom => 135,
            CameraAngle::FrontLeftBottom => 45,
        };

        Quaternion::from_angle_x(Deg(x as f32)) * Quaternion::from_angle_y(Deg(y as f32))
    }
}

#[derive(Clone, Debug)]
pub struct PointerButtonDown {
    pos: Pos2,
    button: PointerButton,
    down_at: std::time::Instant,
    modifiers: eframe::egui::Modifiers,
    scene_object: Option<ColorId>,
}

pub struct ObjectScene {
    id_source: ColorIdSource,
    rotation: AnimatedValue<Quaternion<f32>>,
    scene: Arc<Mutex<Scene>>,
    scene_rect: egui::Rect,
    pointer_buttons_down: Vec<PointerButtonDown>,
    clicked: Option<ColorId>,
}
impl ObjectScene {
    pub fn new(gl: GlowContext) -> Self {
        let mut id_source = ColorIdSource::new();

        Self {
            rotation: AnimatedValue::new(CameraAngle::FrontRightTop.get_rotation()),
            scene: Arc::new(Mutex::new(Scene::new(gl.clone(), &mut id_source))),
            id_source,
            scene_rect: egui::Rect {
                min: (0.0, 0.0).into(),
                max: (0.0, 0.0).into(),
            },
            pointer_buttons_down: Vec::new(),
            clicked: None,
        }
    }
}
