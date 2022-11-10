use super::Pane;

pub struct FeaturesPane {}
impl FeaturesPane {
    pub fn new() -> Self {
        Self {}
    }
}
impl Pane for FeaturesPane {
    fn title(&self) -> String {
        "Features".to_owned()
    }

    fn show(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("This is where the part's feature timeline will be shown".to_owned());
    }
}
