use eframe::egui::{InnerResponse, Ui};

pub fn show(ui: &mut Ui) -> InnerResponse<()> {
    ui.horizontal(|ui| {
        ui.label("Status 1");
        ui.separator();
        ui.label("Status 2");
        ui.separator();
    })
}
