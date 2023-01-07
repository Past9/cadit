use eframe::egui::{self, Ui};
use egui_dock::NodeIndex;
use widgets::editors::{assembly::AssemblyEditor, part::PartEditor, Editor};

use crate::ui::MessageBus;

use self::features::FeaturesPane;

use super::workspace::PaneToAdd;

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
    fn show(&mut self, ui: &mut Ui);
}

pub(super) struct PaneViewer<'a> {
    pub messages: &'a mut MessageBus,
    pub panes_to_add: &'a mut Vec<PaneToAdd>,
}
impl<'a> egui_dock::TabViewer for PaneViewer<'a> {
    type Tab = PaneView;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.pane.show(ui);
        //tab.show(ui);
    }

    fn context_menu(&mut self, ui: &mut Ui, _tab: &mut Self::Tab) {
        ui.label("Add tab context menu this is some really long text foo bar baz blah");
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.pane.title().into()
    }

    fn add_popup(&mut self, ui: &mut Ui, node: NodeIndex) {
        ui.set_min_width(150.0);

        ui.style_mut().visuals.button_frame = false;

        if ui.button("Part editor").clicked() {
            self.panes_to_add
                .push(PaneToAdd::new(node, EditorPane::part()));
        }

        if ui.button("Assembly editor").clicked() {
            self.panes_to_add
                .push(PaneToAdd::new(node, EditorPane::assembly()));
        }

        if ui.button("Features").clicked() {
            self.panes_to_add
                .push(PaneToAdd::new(node, FeaturesPane::new()))
        }
    }
}

pub struct EditorPane {
    editor: Box<dyn Editor>,
}
impl EditorPane {
    pub fn part() -> Self {
        Self {
            editor: Box::new(PartEditor::new()),
        }
    }

    pub fn assembly() -> Self {
        Self {
            editor: Box::new(AssemblyEditor::new()),
        }
    }
}
impl Pane for EditorPane {
    fn title(&self) -> String {
        self.editor.title()
    }

    fn show(&mut self, ui: &mut eframe::egui::Ui) {
        self.editor.show(ui);

        /*
        if let Some(obj) = self.editor.clicked() {
            if let Some(rotation) = CameraAngle::from_name(&obj.name) {
                println!("Click {}", obj.name);
                self.editor.animate_rotation(rotation.get_rotation());
            }
        }
        */
    }
}
