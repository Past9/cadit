use three_d::Quaternion;

use self::{
    assembly::AssemblyEditor,
    part::{ColorId, PartEditor, SceneObject, SceneObjectProps},
};
use super::Pane;
use crate::ui::{organisms::panes::editor::part::CameraAngle, GlowContext};

pub mod assembly;
//pub mod file;
pub mod part;

trait Editor {
    fn title(&self) -> String;
    fn show(&mut self, ui: &mut eframe::egui::Ui);
    fn clicked(&self) -> Option<SceneObjectProps>;
    fn set_rotation(&mut self, rotation: Quaternion<f32>);
    fn animate_rotation(&mut self, rotation: Quaternion<f32>);
}

pub struct EditorPane {
    editor: Box<dyn Editor>,
}
impl EditorPane {
    pub fn part(gl: GlowContext) -> Self {
        Self {
            editor: Box::new(PartEditor::new(gl)),
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

        if let Some(obj) = self.editor.clicked() {
            let rotation = CameraAngle::from_name(&obj.name);
            println!("Click {}", obj.name);
            self.editor.animate_rotation(rotation.get_rotation());
        }
    }
}
