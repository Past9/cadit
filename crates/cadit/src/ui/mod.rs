use self::organisms::workspace::Workspace;
use self::organisms::{menu, status_bar};
use eframe::egui::{self};
use egui_modal::Modal;
use std::collections::VecDeque;
use std::sync::Arc;
use components::{Gui, Window};

mod organisms;

const MENU_HEIGHT: f32 = 17.0;
const STATUS_BAR_HEIGHT: f32 = 21.0;

type GlowContext = Arc<eframe::glow::Context>;

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

    fn handle_messages(&mut self) {
        while let Some(message) = self.messages.pop() {
            match message {
                UiMessage::ErrorDialog(content) => self.error_dialog = Some(content),
            }
        }
    }

    fn show_dialogs(&mut self, ctx: &egui::Context) {
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
impl Window for CaditUi {
    fn draw(&mut self, gui: &mut Gui) {
        gui.immediate_ui(|gui| {
            let ctx = &gui.egui_ctx;
            egui::TopBottomPanel::top("menu")
                .height_range(MENU_HEIGHT..=MENU_HEIGHT)
                .show(ctx, |ui| menu::show(ui));

            egui::TopBottomPanel::bottom("status_bar")
                .height_range(STATUS_BAR_HEIGHT..=STATUS_BAR_HEIGHT)
                .show(ctx, |ui| status_bar::show(ui));

            egui::CentralPanel::default().show(ctx, |ui| {
                self.workspace.show(ctx, ui, &mut self.messages);
            });

            self.handle_messages();
            self.show_dialogs(ctx);
        });
    }

    fn on_close(&mut self) -> bool {
        return true;
    }
}
