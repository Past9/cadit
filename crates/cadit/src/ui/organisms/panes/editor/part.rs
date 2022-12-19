use crate::ui::{
    atoms::{gizmo::Gizmo, object_scene::ObjectScene, scene::SceneObjectProps},
    GlowContext,
};
use eframe::{
    egui::{self, Layout},
    emath::{self, Align},
    epaint::Rect,
};
use three_d::*;

use super::Editor;

pub struct PartEditor {
    gizmo: Gizmo,
    scene: ObjectScene,
}
impl PartEditor {
    pub fn new(gl: GlowContext) -> Self {
        let mut gizmo = Gizmo::new(gl.clone());
        let rotation = gizmo.rotation();
        Self {
            gizmo,
            scene: ObjectScene::new(gl, rotation, vec2(0.0, 0.0), true, true),
        }
    }
}
impl Editor for PartEditor {
    fn title(&self) -> String {
        "Part editor".to_owned()
    }

    fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.scene.set_rotation(rotation);
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        self.scene.show(ui);

        if self.scene.rotated() {
            self.gizmo.set_rotation(self.scene.rotation());
        } else {
            self.scene.set_rotation(self.gizmo.rotation());
        }

        let scene_rect = self.scene.rect();
        let mut gizmo_ui = ui.child_ui(
            Rect::from_min_size(
                scene_rect.left_top() + egui::Vec2 { x: 500.0, y: 200.0 },
                emath::vec2(200.0, 200.0),
            ),
            Layout::top_down(Align::TOP),
        );

        self.gizmo.show(&mut gizmo_ui);
    }

    fn clicked(&self) -> Option<SceneObjectProps> {
        self.scene.clicked()
    }
}
