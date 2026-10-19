use std::path::PathBuf;
use rfd::FileDialog;

use crate::{
    models::mem_card::MemoryCard,
    sunshine_lib::enums::{
        file_error::FileError,
        region::Region
    },
};

/// Launches a file picker for GCI memory card files
/// 
/// Returns a memory card struct
pub fn pick_file() -> MemoryCard {
    let result_path = FileDialog::new()
        .add_filter("Gamecube Saves", &["gci"])
        .set_directory("/")
        .pick_file();

    let path: String = match result_path {
        Some(pb) => pb.display().to_string(),
        None => String::from(""),
    };

    if path == "" {
        return MemoryCard { mem_data: vec![], file_error: FileError::ReadCancelled, region: Region::INVALID };
    }

    let mem_card: MemoryCard = MemoryCard::read_from_file(&path);

    mem_card.clone()
}

/// Launches a file dialog to pick a directory to save a txt file to
/// 
/// Returns the path as an option
pub fn pick_directory() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Text File", &["txt"])
        .set_title("Select where to write the report to")
        .save_file()
}