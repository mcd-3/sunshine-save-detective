use crate::enums::file_error::FileError;
use crate::sunshine_lib::enums::world_identifier::WorldIdentifier;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    FileSelectOpen,
    DisplayFileError(FileError),
    SelectSaveFile(u8),
    SelectWorld(WorldIdentifier),
    FileReportWriteToOpen,
}