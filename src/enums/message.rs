use crate::enums::{
    file_error::FileError,
    world_identifier::WorldIdentifier
};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Default,
    FileSelectOpen,
    DisplayFileError(FileError),
    SelectSaveFile(u8),
    SelectWorld(WorldIdentifier),
}