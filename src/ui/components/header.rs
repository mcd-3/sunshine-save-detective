use iced::{widget::{row, text, Row}, Font};
use iced::font::Weight::Semibold;
use crate::enums::message::Message;

pub fn header<'a>(str: String) -> Row<'a, Message> {
    row![
        text(str).font(
            Font {
                weight: Semibold,
                ..Font::DEFAULT
            }
        ),
    ].padding([10, 0])
}