use crate::{gizmo::Gizmo, scene::SceneViewer};
use cgmath::{vec3, Quaternion};
use eframe::egui;
use render::scene::Scene;

use super::Editor;

pub struct PartEditor {
    gizmo: Gizmo,
    viewer: SceneViewer,
}
impl PartEditor {
    pub fn new(scene: Scene) -> Self {
        /*
        let mut gizmo = Gizmo::new(scene.clone());
        let rotation = gizmo.rotation();
        Self {
            gizmo,
            viewer: SceneViewer::new(rotation, vec3(0.0, 0.0, 0.0), true, true, true, scene),
        }
        */
        todo!()
    }
}
impl Editor for PartEditor {
    fn title(&self) -> String {
        "Part editor".to_owned()
    }

    fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.viewer.set_rotation(rotation);
    }

    fn show(&mut self, ui: &mut egui::Ui) {
        self.viewer.show(ui);

        if self.viewer.rotated() {
            self.gizmo.set_rotation(self.viewer.rotation());
        } else {
            self.viewer.set_rotation(self.gizmo.rotation());
        }

        /*
        let scene_rect = self.scene.rect();
        let mut gizmo_ui = ui.child_ui(
            Rect::from_min_size(
                scene_rect.left_top() + egui::Vec2 { x: 500.0, y: 200.0 },
                emath::vec2(200.0, 200.0),
            ),
            Layout::top_down(Align::TOP),
        );

        self.gizmo.show(&mut gizmo_ui);
        */
    }

    /*
    fn clicked(&self) -> Option<SceneObjectProps> {
        self.scene.clicked()
    }
    */
}
