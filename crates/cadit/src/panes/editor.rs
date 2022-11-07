use std::path::PathBuf;

use crate::{error::CaditResult, file::CaditFile};

use super::Pane;

pub struct EditorPane {
    file: CaditFile,
}
impl EditorPane {
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
        Self { file }
    }
}
impl Pane for EditorPane {
    fn title(&self) -> String {
        self.file.file_name().to_string_lossy().to_string()
    }

    fn show(&self, ui: &mut eframe::egui::Ui) {
        let label_text = match self.file.file_type() {
            crate::file::FileType::Part => "Edit the part here",
            crate::file::FileType::Assembly => "Edit the assembly here",
        };

        ui.label(label_text.to_owned());
    }
}
