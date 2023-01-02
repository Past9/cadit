use std::time::Duration;

use cgmath::{Deg, Quaternion, Rotation3, Vector2};
use eframe::egui;

use crate::{render::cgmath_types::vec3, ui::math::AnimatedValue};

use super::object_scene::ObjectScene;

#[derive(Debug, Clone, Copy)]
pub enum CameraAngle {
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

pub struct Gizmo {
    scene: ObjectScene,
    rotation: AnimatedValue<Quaternion<f32>>,
}
impl Gizmo {
    pub fn new() -> Self {
        let rotation = CameraAngle::Front.get_rotation();
        Self {
            scene: ObjectScene::new(
                rotation,
                vec3(0.0, 0.0, 0.0),
                false,
                false,
                [0.0, 0.0, 0.0, 0.0],
            ),
            rotation: AnimatedValue::new(rotation),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.scene.show(ui);

        self.scene.set_rotation(self.rotation.value());

        if let Some(obj) = self.scene.clicked() {
            if let Some(rotation) = CameraAngle::from_name(&obj.name) {
                self.rotation.clear();
                self.rotation
                    .push_swing(rotation.get_rotation(), Duration::from_millis(500));
            }
        }

        if self.rotation.has_queued() {
            ui.ctx().request_repaint();
        }
    }

    pub fn rotated(&self) -> bool {
        self.scene.rotated()
    }

    pub fn rotation(&mut self) -> Quaternion<f32> {
        self.rotation.value()
    }

    pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation.set_immediate(rotation);
    }
}
