use crate::sunshine_lib::enums::{
    file_error::FileError,
    world_identifier::WorldIdentifier
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    FileSelectOpen,
    DisplayFileError(FileError),
    SelectSaveFile(u8),
    SelectWorld(WorldIdentifier),
    FileReportWriteToOpen,
}