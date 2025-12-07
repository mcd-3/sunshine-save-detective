use iced::{Alignment::Center, Length, widget::{Column, Row, radio, row, text}};

use crate::{enums::message::Message, models::save_file::SaveFile};

const DEFAULT_SAVE_FILE_INDEX: u8 = SaveFile::SAVE_A_FILE_INDEX + 1;
const ROW_HEIGHT: u16 = 32;

pub fn save_radio<'a>(selected_save_index: u8) -> Row<'a, Message> {
    row![
        text("Save File: "),
        radio(
            format!("{} ", SaveFile::SAVE_A_LABEL),
            DEFAULT_SAVE_FILE_INDEX,
            Some(selected_save_index + 1),
            |i| Message::SelectSaveFile(i)
        ),
        radio(
            format!("{} ", SaveFile::SAVE_B_LABEL),
            DEFAULT_SAVE_FILE_INDEX + 1,
            Some(selected_save_index + 1),
            |i| Message::SelectSaveFile(i)
        ),
        radio(
            SaveFile::SAVE_C_LABEL,
            DEFAULT_SAVE_FILE_INDEX + 2,
            Some(selected_save_index + 1),
            |i| Message::SelectSaveFile(i)
        )
    ].width(Length::FillPortion(1))
        .height(ROW_HEIGHT)
        .align_y(Center)
}