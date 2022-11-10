use eframe::egui;

pub struct AssemblyEditorState {}
impl AssemblyEditorState {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct AssemblyEditor<'a> {
    pub(crate) state: &'a mut AssemblyEditorState,
}
impl<'a> AssemblyEditor<'a> {
    pub fn with_state(state: &'a mut AssemblyEditorState) -> Self {
        Self { state }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.label("Edit the assembly here");
    }
}
