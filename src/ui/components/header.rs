use iced::{widget::{row, text, Row}, Font};
use iced::font::Weight::Semibold;
use crate::ui::views::app::Message;

pub fn header(str: String) -> Row<'static, Message> {
    row![
        text(str).font(
            Font {
                weight: Semibold,
                ..Font::DEFAULT
            }
        ),
    ].padding([10, 0])
}