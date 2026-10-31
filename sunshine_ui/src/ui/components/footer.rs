use iced::{
    Length::{self, Shrink},
    alignment::Horizontal::{Left, Right},
    widget::{Row, row, text}
};

use crate::enums::message::Message;

const TEXT_SIZE_SUBHEADER: u16 = 12;

pub fn footer<'a>() -> Row<'a, Message> {
    let version: String = String::from("Version: v.") + env!["CARGO_PKG_VERSION"];
    let author: String = String::from("By: mcd-3");

    row![
        text(author)
            .size(TEXT_SIZE_SUBHEADER)
            .width(Length::FillPortion(1))
            .align_x(Left),
        text(version.clone())
            .size(TEXT_SIZE_SUBHEADER)
            .width(Length::FillPortion(1))
            .align_x(Right),
    ].height(Shrink)
}