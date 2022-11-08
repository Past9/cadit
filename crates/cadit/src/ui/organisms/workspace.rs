use eframe::egui::{self, Ui};
use egui_dock::{DockArea, NodeIndex, StyleBuilder, Tree};

use crate::{
    ui::MessageBus,
    ui::{
        organisms::panes::{editor::EditorPane, Pane},
        UiMessage,
    },
};

use super::panes::{features::FeaturesPane, PaneView};

struct PaneToAdd {
    parent_node: NodeIndex,
    pane: PaneView,
}
impl PaneToAdd {
    pub fn new(parent_node: NodeIndex, pane: impl Pane + 'static) -> Self {
        Self {
            parent_node,
            pane: PaneView::new(pane),
        }
    }
}

pub(crate) struct Workspace {
    tree: Tree<PaneView>,
}
impl Workspace {
    pub fn new() -> Self {
        let mut tree = Tree::new(vec![PaneView::new(
            EditorPane::from_path_str("some/awesome/WidgetPart.cpt").unwrap(),
        )]);
        tree.split_left(
            NodeIndex::root(),
            0.15,
            vec![PaneView::new(FeaturesPane::new())],
        );

        Self { tree }
    }

    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, messages: &mut MessageBus) {
        let mut panes_to_add = Vec::new();
        DockArea::new(&mut self.tree)
            .style(
                StyleBuilder::from_egui(ctx.style().as_ref())
                    .show_add_buttons(true)
                    .show_context_menu(true)
                    .show_add_popup(true)
                    .build(),
            )
            .show(
                ui.ctx(),
                &mut PaneViewer {
                    messages,
                    panes_to_add: &mut panes_to_add,
                },
            );

        panes_to_add.drain(..).for_each(|node| {
            self.tree.set_focused_node(node.parent_node);
            self.tree.push_to_focused_leaf(node.pane);
        });
    }
}

struct PaneViewer<'a> {
    messages: &'a mut MessageBus,
    panes_to_add: &'a mut Vec<PaneToAdd>,
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

        if ui.button("Editor").clicked() {
            match EditorPane::from_path_str("my/cool/CustomPart.cptx") {
                Ok(editor) => {
                    self.panes_to_add.push(PaneToAdd::new(node, editor));
                }
                Err(err) => {
                    self.messages.push(UiMessage::ErrorDialog(err.to_string()));
                }
            };
        }

        if ui.button("Features").clicked() {
            self.panes_to_add
                .push(PaneToAdd::new(node, FeaturesPane::new()))
        }
    }
}
