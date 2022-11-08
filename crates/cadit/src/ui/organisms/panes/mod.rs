use eframe::egui::Ui;

pub mod editor;
pub mod explorer;
pub mod features;

pub struct PaneView {
    pub pane: Box<dyn Pane>,
}
impl PaneView {
    pub fn new(pane: impl Pane + 'static) -> Self {
        Self {
            pane: Box::new(pane),
        }
    }
}

pub trait Pane {
    fn title(&self) -> String;
    fn show(&self, ui: &mut Ui);
}
