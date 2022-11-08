use eframe::egui::{InnerResponse, Ui};

pub fn show(ui: &mut Ui, frame: &mut eframe::Frame) -> InnerResponse<()> {
    ui.horizontal(|ui| {
        ui.style_mut().visuals.button_frame = false;

        ui.menu_button("File", |ui| {
            if ui.button("Open folder").clicked() {
                println!("opening folder");
            }

            ui.separator();

            if ui.button("Save").clicked() {
                println!("Saving file");
            }

            if ui.button("Save As...").clicked() {
                println!("Saving file as");
            }

            if ui.button("Save All").clicked() {
                println!("Saving all files");
            }

            ui.separator();

            if ui.button("Close folder").clicked() {
                println!("Closing folder");
            }

            ui.separator();

            if ui.button("Exit").clicked() {
                println!("Exiting application");
                frame.close();
            }
        });
        ui.menu_button("Edit", |ui| if ui.button("Some edit stuff").clicked() {});
        ui.menu_button(
            "Window",
            |ui| if ui.button("Some window stuff").clicked() {},
        );
        ui.menu_button("Help", |ui| if ui.button("Some help stuff").clicked() {});

        ui.separator();

        ui.label("Layouts:");

        if ui.button("Part").clicked() {
            println!("Switch to part layout");
        }

        if ui.button("Assembly").clicked() {
            println!("Switch to assembly layout");
        }

        if ui.button("Simulate").clicked() {
            println!("Switch to simulation layout");
        }

        if ui.button("Animate").clicked() {
            println!("Switch to animation layout");
        }

        if ui.button("Render").clicked() {
            println!("Switch to render layout");
        }
    })
}
