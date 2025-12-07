use iced::{Color, widget::{Text, text}};

use crate::{
    enums::file_error::FileError
};

pub fn error_warning(file_error: &FileError) -> Text {
    let error_text: &str = match file_error {
        FileError::FileRead => "Cannot read file",
        FileError::NotProperGame => "Game save is not from Super Mario Sunshine",
        FileError::NotProperRegion => "Game save is not from the North America region",
        FileError::NotValidSave => "Not a valid GCI file",
        _ => ""
    };

    text(error_text).color(
        Color::from_rgb(255.0, 0.0, 0.0)
    )
}