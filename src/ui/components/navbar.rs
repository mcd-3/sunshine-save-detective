use iced::{
    Alignment::Center,
    Font,
    Length::{self, Fill, Shrink},
    alignment::Horizontal::Left,
    widget::{Row, button, row, text},
};

use crate::{
    enums::message::Message,
    ui::components::icon::icon_large,
    utils::{
        assets::sunshine_detective_svg_path
    },
    sunshine_lib::utils::string::padded_string
};

const TEXT_SIZE_HEADER: u16 = 24;

pub fn navbar<'a>(hide_button: bool) -> Row<'a, Message> {
    if hide_button {
        return row![
            icon_large(sunshine_detective_svg_path()),
            text(
                padded_string("Sunshine Save Detective", 2, 0)
            ).size(TEXT_SIZE_HEADER)
                .width(Length::FillPortion(1))
                .height(Fill)
                .align_x(Left)
                .align_y(Center)
                .font(Font::with_name("Knewave")),
        ].height(Shrink)
    } else {
        return row![
            icon_large(sunshine_detective_svg_path()),
            text(
                padded_string("Sunshine Save Detective", 2, 0)
            ).size(TEXT_SIZE_HEADER)
                .width(Length::FillPortion(1))
                .height(Fill)
                .align_x(Left)
                .align_y(Center)
                .font(Font::with_name("Knewave")),
            row![
                button("Record")
                    .padding([10, 20])
                    .on_press(Message::FileReportWriteToOpen),
                button("Upload")
                    .padding([10, 20])
                    .on_press(Message::FileSelectOpen),
            ].spacing(10)
        ].height(Shrink)
    }
}