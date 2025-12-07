use iced::{
    Alignment::Center,
    Length::{self, Fill, Shrink},
    alignment::Horizontal::Left,
    widget::{Row, row, text}
};

use crate::{enums::message::Message, ui::components::icon::icon_large};

const TEXT_SIZE_HEADER: u16 = 18;

pub fn navbar<'a>() -> Row<'a, Message> {
    row![
        icon_large("src/assets/sunshine_detective.svg"),
        text(format!("  {}", "Sunshine Save Detective"))
            .size(TEXT_SIZE_HEADER)
            .width(Length::FillPortion(1))
            .height(Fill)
            .align_x(Left)
            .align_y(Center),
    ].height(Shrink)
}