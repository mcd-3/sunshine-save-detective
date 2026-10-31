use iced::{Color, widget::{Text, text}};

use crate::sunshine_lib::enums::file_error::FileError;

pub fn error_warning(file_error: &FileError) -> Text<'_> {
    let error_text: &str = match file_error {
        FileError::FileRead => "Cannot read file",
        FileError::NotProperGame => "Game save is not from Super Mario Sunshine",
        FileError::NotValidSave => "Not a valid GCI file",
        _ => ""
    };

    text(error_text).color(
        Color::from_rgb(255.0, 0.0, 0.0)
    )
}