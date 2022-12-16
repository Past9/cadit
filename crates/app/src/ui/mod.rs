mod organisms;

use cadit::ui::MessageBus;

pub struct CaditUi {
    messages: MessageBus,
    workspace: Workspace,
    error_dialog: Option<String>,
}
impl CaditUi {
    pub fn run() {
        let mut options = NativeOptions::default();
        options.initial_window_size = Some(Vec2::new(1760.0, 990.0));
        eframe::run_native(
            "Cadit",
            options,
            Box::new(|cc| {
                let gl = cc.gl.clone().unwrap();
                Box::new(Self {
                    messages: MessageBus::new(),
                    workspace: Workspace::new(gl),
                    error_dialog: None,
                })
            }),
        );
    }

    /*
    pub fn run(self) {
        let mut options = NativeOptions::default();
        options.initial_window_size = Some(Vec2::new(1760.0, 990.0));
        eframe::run_native("Cadit", options, Box::new(|_cc| Box::new(self)));
    }
    */

    fn handle_messages(&mut self) {
        while let Some(message) = self.messages.pop() {
            match message {
                UiMessage::ErrorDialog(content) => self.error_dialog = Some(content),
            }
        }
    }

    fn show_dialogs(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(error_dialog) = self.error_dialog.clone() {
            let err_modal = Modal::new(ctx, "error_modal");

            err_modal.show(|ui| {
                err_modal.title(ui, "Error");
                err_modal.frame(ui, |ui| {
                    err_modal.body(ui, error_dialog);
                });
                err_modal.buttons(ui, |ui| {
                    if err_modal.button(ui, "Ok").clicked() {
                        self.error_dialog = None;
                    }
                });
            });

            err_modal.open();
        }
    }
}
impl eframe::App for CaditUi {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu")
            .height_range(MENU_HEIGHT..=MENU_HEIGHT)
            .show(ctx, |ui| menu::show(ui, frame));

        egui::TopBottomPanel::bottom("status_bar")
            .height_range(STATUS_BAR_HEIGHT..=STATUS_BAR_HEIGHT)
            .show(ctx, |ui| status_bar::show(ui));

        egui::CentralPanel::default().show(ctx, |ui| {
            self.workspace.show(ctx, ui, &mut self.messages);
        });

        self.handle_messages();
        self.show_dialogs(ctx, frame);
    }
}
