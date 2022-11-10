use self::{assembly::AssemblyEditor, file::CaditFile, part::PartEditor};

use super::Pane;
use crate::error::CaditResult;
use std::path::PathBuf;

pub mod assembly;
pub mod file;
pub mod part;

pub struct EditorPane {
    file: Option<CaditFile>,
}
impl EditorPane {
    pub fn empty() -> Self {
        Self { file: None }
    }

    pub fn from_path_str(file_path: &str) -> CaditResult<Self> {
        Ok(Self::from_file(CaditFile::attach(PathBuf::from(
            file_path,
        ))?))
    }

    #[allow(dead_code)]
    pub fn from_path(file_path: PathBuf) -> CaditResult<Self> {
        Ok(Self::from_file(CaditFile::attach(file_path)?))
    }

    pub fn from_file(file: CaditFile) -> Self {
        Self { file: Some(file) }
    }
}
impl Pane for EditorPane {
    fn title(&self) -> String {
        match &self.file {
            Some(file) => file.file_name().to_string_lossy().to_string(),
            None => "Untitled".to_owned(),
        }
    }

    fn show(&mut self, ui: &mut eframe::egui::Ui) {
        match &mut self.file {
            Some(file) => match file {
                CaditFile::Part(file) => PartEditor::with_state(&mut file.editor_state).show(ui),
                CaditFile::Assembly(file) => {
                    AssemblyEditor::with_state(&mut file.editor_state).show(ui)
                }
            },

            /*
            Some(file) => match file.file_type() {
                crate::file::FileType::Part => {
                    PartEditor::new().show(ui);
                }
                crate::file::FileType::Assembly => {
                    ui.label("Edit the assembly here");
                }
            },
            */
            None => {
                ui.label("No file");
            }
        };
    }
}
