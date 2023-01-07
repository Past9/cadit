use eframe::egui::{self};
use egui_dock::{DockArea, NodeIndex, StyleBuilder, Tree};

use crate::{ui::organisms::panes::Pane, ui::MessageBus};

use super::panes::{features::FeaturesPane, EditorPane, PaneView, PaneViewer};

pub(super) struct PaneToAdd {
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
        let mut tree = Tree::new(vec![PaneView::new(EditorPane::part())]);
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
