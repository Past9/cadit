use eframe::{
    egui::{self, Ui},
    epaint::Vec2,
    NativeOptions,
};
use egui_dock::{DockArea, NodeIndex, StyleBuilder, Tree};
use panes::{editor::EditorPane, features::FeaturesPane, PaneView};

mod error;
mod file;
mod menu;
mod panes;
mod status_bar;

fn main() {
    let mut options = NativeOptions::default();
    options.initial_window_size = Some(Vec2::new(1760.0, 990.0));
    eframe::run_native("Cadit", options, Box::new(|_cc| Box::new(App::default())));
}

struct TabView<'a> {
    added_nodes: &'a mut Vec<(NodeIndex, PaneView)>,
}
impl egui_dock::TabViewer for TabView<'_> {
    type Tab = PaneView;

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.pane.show(ui);
    }

    fn context_menu(&mut self, ui: &mut Ui, _tab: &mut Self::Tab) {
        ui.label("Add tab context menu this is some really long text foo bar baz blah");
    }

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.pane.title().into()
    }

    fn add_popup(&mut self, ui: &mut Ui, node: NodeIndex) {
        ui.set_min_width(150.0);
        ui.label("Add tab menu");
    }
}

struct App {
    tree: Tree<PaneView>,
}
impl Default for App {
    fn default() -> Self {
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
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        const MENU_HEIGHT: f32 = 17.0;
        const STATUS_BAR_HEIGHT: f32 = 21.0;

        egui::TopBottomPanel::top("menu")
            .height_range(MENU_HEIGHT..=MENU_HEIGHT)
            .show(ctx, |ui| menu::show(ui, frame));

        egui::TopBottomPanel::bottom("status_bar")
            .height_range(STATUS_BAR_HEIGHT..=STATUS_BAR_HEIGHT)
            .show(ctx, |ui| status_bar::show(ui));

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut added_nodes = Vec::new();
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
                    &mut TabView {
                        added_nodes: &mut added_nodes,
                    },
                );

            added_nodes.drain(..).for_each(|node| {
                self.tree.set_focused_node(node.0);
                self.tree.push_to_focused_leaf(node.1);
            });
        });
    }
}
