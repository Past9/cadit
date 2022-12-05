use self::{
    assembly::AssemblyEditor,
    part::{ColorId, PartEditor, SceneObject, SceneObjectProps},
};
use super::Pane;
use crate::ui::GlowContext;

pub mod assembly;
//pub mod file;
pub mod part;

trait Editor {
    fn title(&self) -> String;
    fn show(&mut self, ui: &mut eframe::egui::Ui);
    fn clicked(&self) -> Option<SceneObjectProps>;
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
            println!("Click {}", obj.name);
        }
    }
}
