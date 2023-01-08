use std::time::Duration;

use crate::{scene::SceneViewer, util::animation::AnimatedValue};
use cgmath::{vec3, Quaternion};
use eframe::egui;
use render::{camera::CameraAngle, scene::Scene};

pub struct Gizmo {
    viewer: SceneViewer,
    rotation: AnimatedValue<Quaternion<f32>>,
}
impl Gizmo {
    pub fn new(scene: Scene) -> Self {
        let rotation = CameraAngle::Front.get_rotation();
        println!("GIZMO ROT {:?}", rotation);
        Self {
            viewer: SceneViewer::new(rotation, vec3(0.0, 0.0, 0.0), false, false, false, scene),
            rotation: AnimatedValue::new(rotation),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.viewer.show(ui);

        self.viewer.set_rotation(self.rotation.value());

        if let Some(obj) = self.viewer.clicked() {
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
        self.viewer.rotated()
    }

    pub fn rotation(&mut self) -> Quaternion<f32> {
        self.rotation.value()
    }

    pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation.set_immediate(rotation);
    }
}
