use super::{
    part::{SceneObject, SceneObjectProps},
    Editor,
};

pub struct AssemblyEditor {}
impl AssemblyEditor {
    pub fn new() -> Self {
        Self {}
    }
}
impl Editor for AssemblyEditor {
    fn title(&self) -> String {
        "Assembly editor".to_owned()
    }

    fn show(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("Edit assembly here");
    }

    fn clicked(&self) -> Option<SceneObjectProps> {
        None
    }

    fn set_rotation(&mut self, rotation: three_d::Quaternion<f32>) {
        // nothing
    }
}
