use cgmath::{point3, vec3, Deg, InnerSpace};
use eframe::egui::{self, Ui};
use egui_dock::NodeIndex;
use render::{
    camera::Camera,
    lights::DirectionalLight,
    mesh::{Edge, EdgeVertex, Point, Surface, SurfaceVertex},
    model::{Material, Model, ModelEdge, ModelPoint, ModelSurface},
    rgba,
    scene::{Scene, SceneLights},
    Rgb, Rgba,
};
use components::editors::{assembly::AssemblyEditor, part::PartEditor, Editor};

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
            editor: Box::new(PartEditor::new(Scene::new(
                rgba(0.1, 0.2, 0.4, 1.0),
                SceneLights::new(
                    vec![
                        //AmbientLight::new(Rgb::WHITE, 0.05),
                        //AmbientLight::new(Rgb::RED, 0.5),
                    ],
                    vec![
                        DirectionalLight::new(vec3(1.0, 0.0, 1.0).normalize(), Rgb::BLUE, 1.0),
                        DirectionalLight::new(vec3(-1.0, 0.0, 1.0).normalize(), Rgb::YELLOW, 1.0),
                    ],
                    vec![
                        //PointLight::new(point3(3.0, 3.0, -5.0), Rgb::RED, 7.0),
                        //PointLight::new(point3(-3.0, -3.0, -5.0), Rgb::GREEN, 2.0),
                    ],
                ),
                Camera::create_perspective(
                    [0, 0],
                    point3(0.0, 0.0, -5.0),
                    vec3(0.0, 0.0, 1.0),
                    vec3(0.0, -1.0, 0.0).normalize(),
                    Deg(70.0).into(),
                    0.01,
                    5.0,
                ),
                vec![Model::new(
                    vec![ModelSurface::new(
                        0.into(),
                        Surface::new(
                            [
                                SurfaceVertex::new(point3(-0.9, -0.9, 0.0), vec3(0.0, 0.0, -1.0)),
                                SurfaceVertex::new(point3(-0.9, 0.9, 0.0), vec3(0.0, 0.0, -1.0)),
                                SurfaceVertex::new(point3(0.9, -0.9, 0.0), vec3(0.0, 0.0, -1.0)),
                                SurfaceVertex::new(point3(0.6, 0.6, 0.0), vec3(0.0, 0.0, -1.0)),
                            ],
                            [0, 1, 2, 2, 1, 3],
                        ),
                        0,
                    )],
                    vec![ModelEdge::new(
                        0.into(),
                        Edge::new([
                            EdgeVertex::new(point3(-0.9, -0.9, 0.0), vec3(0.0, 0.0, -1.0)),
                            EdgeVertex::new(point3(-0.9, 0.9, 0.0), vec3(0.0, 0.0, -1.0)),
                            EdgeVertex::new(point3(0.9, -0.9, 0.0), vec3(0.0, 0.0, -1.0)),
                            EdgeVertex::new(point3(0.6, 0.6, 0.0), vec3(0.0, 0.0, -1.0)),
                        ]),
                        Rgba::BLACK,
                    )],
                    vec![
                        ModelPoint::new(
                            0.into(),
                            Point::new(point3(-0.9, -0.9, 0.0), vec3(0.0, 0.0, -1.0)),
                            Rgba::WHITE,
                        ),
                        ModelPoint::new(
                            0.into(),
                            Point::new(point3(-0.9, 0.9, 0.0), vec3(0.0, 0.0, -1.0)),
                            Rgba::WHITE,
                        ),
                        ModelPoint::new(
                            0.into(),
                            Point::new(point3(0.9, -0.9, 0.0), vec3(0.0, 0.0, -1.0)),
                            Rgba::WHITE,
                        ),
                        ModelPoint::new(
                            0.into(),
                            Point::new(point3(0.6, 0.6, 0.0), vec3(0.0, 0.0, -1.0)),
                            Rgba::WHITE,
                        ),
                    ],
                )],
                vec![Material::new(rgba(1.0, 1.0, 1.0, 1.0), 0.5)],
            ))),
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
