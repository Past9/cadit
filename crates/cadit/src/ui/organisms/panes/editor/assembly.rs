use super::Editor;

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
}