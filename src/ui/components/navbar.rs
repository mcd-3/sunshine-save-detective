use iced::{
    Alignment::Center,
    Font,
    Length::{self, Fill, Shrink},
    alignment::Horizontal::Left,
    widget::{Row, row, text},
};

use crate::{
    enums::message::Message,
    ui::components::icon::icon_large,
    utils::{
        assets::sunshine_detective_svg_path,
        string::padded_string
    }
};

const TEXT_SIZE_HEADER: u16 = 24;

pub fn navbar<'a>() -> Row<'a, Message> {
    row![
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
}