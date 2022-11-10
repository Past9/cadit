use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

use crate::error::{CaditError, CaditResult};

use super::{assembly::AssemblyEditorState, part::PartEditorState};

const PART_FILE_EXT: &str = "cdp";
const ASSEMBLY_FILE_EXT: &str = "cda";

pub struct PartFileState {
    file_path: PathBuf,
    file_name: OsString,
    pub(crate) editor_state: PartEditorState,
}

pub struct AssemblyFileState {
    file_path: PathBuf,
    file_name: OsString,
    pub(crate) editor_state: AssemblyEditorState,
}

pub enum CaditFile {
    Part(PartFileState),
    Assembly(AssemblyFileState),
}
impl CaditFile {
    pub fn from_part_file(file_path: PathBuf) -> CaditResult<Self> {
        Ok(Self::Part(PartFileState {
            file_name: Self::file_name_from_path(&file_path)?,
            file_path,
            editor_state: PartEditorState::new(),
        }))
    }

    pub fn from_assembly_file(file_path: PathBuf) -> CaditResult<Self> {
        Ok(Self::Assembly(AssemblyFileState {
            file_name: Self::file_name_from_path(&file_path)?,
            file_path,
            editor_state: AssemblyEditorState::new(),
        }))
    }

    fn file_name_from_path(file_path: &Path) -> CaditResult<OsString> {
        match file_path.file_name() {
            Some(file_name) => Ok(file_name.to_owned()),
            None => Err(CaditError::AttemptToOpenDirectoryAsFile(
                file_path.to_owned(),
            )),
        }
    }

    pub fn file_path(&self) -> &Path {
        match self {
            CaditFile::Part(state) => &state.file_path,
            CaditFile::Assembly(state) => &state.file_path,
        }
    }

    pub fn file_name(&self) -> &OsStr {
        match self {
            CaditFile::Part(state) => &state.file_name,
            CaditFile::Assembly(state) => &state.file_name,
        }
    }

    pub fn is_part(&self) -> bool {
        match self {
            CaditFile::Part(_) => true,
            _ => false,
        }
    }

    pub fn is_assembly(&self) -> bool {
        match self {
            CaditFile::Assembly(_) => true,
            _ => false,
        }
    }

    pub fn attach(file_path: PathBuf) -> CaditResult<Self> {
        let ext = file_path.extension();

        match ext {
            Some(ext) => match ext.to_ascii_lowercase().to_str() {
                Some(ext) => match ext {
                    PART_FILE_EXT => Self::from_part_file(file_path),
                    ASSEMBLY_FILE_EXT => Self::from_assembly_file(file_path),
                    _ => {
                        return Err(CaditError::InvalidFileExtension(ext.to_owned()));
                    }
                },
                None => return Err(CaditError::UnreadableFileExtension(ext.to_owned())),
            },
            None => return Err(CaditError::MissingFileExtension),
        }
    }
}

/*

#[derive(Clone, Debug, PartialEq)]
pub enum FileType {
    Part,
    Assembly,
}
impl FileType {
    pub fn is_part(&self) -> bool {
        self == &Self::Part
    }

    pub fn is_assembly(&self) -> bool {
        self == &Self::Assembly
    }
}

pub struct CaditFile {
    #[allow(dead_code)]
    file_path: PathBuf,
    file_name: OsString,
    file_type: FileType,
}
impl CaditFile {
    pub fn is_part(&self) -> bool {
        self.file_type.is_part()
    }

    pub fn is_assembly(&self) -> bool {
        self.file_type.is_assembly()
    }

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
*/
