use iced::{
    Alignment::Center,
    Font,
    Length::Fill,
    widget::{Row, button, column, row, text},
    font::Weight::Semibold
};

use crate::{
    enums::{
        file_error::FileError,
        message::Message
    },
    ui::components::error_warning::error_warning
};

const DEFAULT_SPACING: u16 = 16;

pub fn file_prompter(file_error: &FileError) -> Row<'_, Message>{
    row![
        column![
            text("Select a GCI file")
                .size(22)
                .font(
                    Font {
                        weight: Semibold,
                        ..Font::DEFAULT
                    }
                ),
            button("Upload")
                .padding([10, 20])
                .on_press(Message::FileSelectOpen),
            error_warning(file_error),
        ].align_x(Center)
            .spacing(DEFAULT_SPACING)
    ].height(Fill)
        .align_y(Center)
}