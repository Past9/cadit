use self::organisms::workspace::Workspace;
use self::organisms::{menu, status_bar};
use eframe::egui::{self};
use eframe::{epaint::Vec2, NativeOptions};
use std::collections::VecDeque;

mod atoms;
mod molecules;
mod organisms;

const MENU_HEIGHT: f32 = 17.0;
const STATUS_BAR_HEIGHT: f32 = 21.0;

pub(crate) enum UiMessage {
    ErrorDialog(String),
}

pub(crate) struct MessageBus {
    messages: VecDeque<UiMessage>,
}
impl MessageBus {
    pub fn new() -> Self {
        Self {
            messages: VecDeque::new(),
        }
    }

    pub fn push(&mut self, message: UiMessage) {
        self.messages.push_front(message);
    }

    pub fn pop(&mut self) -> Option<UiMessage> {
        self.messages.pop_back()
    }
}

pub struct CaditUi {
    messages: MessageBus,
    workspace: Workspace,
    error_dialog: Option<String>,
}
impl CaditUi {
    pub fn new() -> Self {
        Self {
            messages: MessageBus::new(),
            workspace: Workspace::new(),
            error_dialog: None,
        }
    }

    pub fn run(self) {
        let mut options = NativeOptions::default();
        options.initial_window_size = Some(Vec2::new(1760.0, 990.0));
        eframe::run_native("Cadit", options, Box::new(|_cc| Box::new(self)));
    }

    fn handle_messages(&mut self) {
        while let Some(message) = self.messages.pop() {
            match message {
                UiMessage::ErrorDialog(content) => self.error_dialog = Some(content),
            }
        }
    }

    fn show_dialogs(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(ref error_dialog) = self.error_dialog {
            egui::Window::new(error_dialog)
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Ok").clicked() {
                            self.error_dialog = None;
                        }
                    });
                });
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
