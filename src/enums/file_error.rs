#[derive(Debug, Clone, Copy)]
pub enum FileError {
    NoError,
    FileRead,
    NotProperGame,
    NotValidSave,
    NotProperRegion,
    ReadCancelled
}

impl Default for FileError {
    fn default() -> Self {
        FileError::NoError
    }
}