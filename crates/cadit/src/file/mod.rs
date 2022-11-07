use std::{
    ffi::{OsStr, OsString},
    path::PathBuf,
};

use crate::error::{CaditError, CaditResult};

const PART_FILE_EXT: &str = "cpt";
const ASSEMBLY_FILE_EXT: &str = "cas";

pub enum FileType {
    Part,
    Assembly,
}

pub struct CaditFile {
    #[allow(dead_code)]
    file_path: PathBuf,
    file_name: OsString,
    file_type: FileType,
}
impl CaditFile {
    pub fn attach(file_path: PathBuf) -> CaditResult<Self> {
        let ext = file_path.extension();

        let file_name = match file_path.file_name() {
            Some(file_name) => file_name.to_owned(),
            None => return Err(CaditError::AttemptToOpenDirectoryAsFile(file_path)),
        };

        let file_type = match ext {
            Some(ext) => match ext.to_ascii_lowercase().to_str() {
                Some(ext) => match ext {
                    PART_FILE_EXT => FileType::Part,
                    ASSEMBLY_FILE_EXT => FileType::Assembly,
                    _ => {
                        return Err(CaditError::InvalidFileExtension(ext.to_owned()));
                    }
                },
                None => return Err(CaditError::UnreadableFileExtension(ext.to_owned())),
            },
            None => return Err(CaditError::MissingFileExtension),
        };

        Ok(Self {
            file_path,
            file_name,
            file_type,
        })
    }

    pub fn file_type(&self) -> &FileType {
        &self.file_type
    }

    pub fn file_name(&self) -> &OsStr {
        &self.file_name
    }
}
