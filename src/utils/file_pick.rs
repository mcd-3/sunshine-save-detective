use rfd::FileDialog;

use crate::{
    enums::file_error::FileError,
    models::mem_card::MemoryCard
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
        return MemoryCard { mem_data: vec![], file_error: FileError::ReadCancelled };
    }

    let mem_card: MemoryCard = MemoryCard::read_from_file(&path);

    mem_card.clone()
}