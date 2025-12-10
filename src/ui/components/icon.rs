use iced::{Theme, advanced::svg::Handle, widget::{Container, Svg, Tooltip, container, text, tooltip::Position}};

use crate::enums::message::Message;

const SVG_SIZE: u16 = 28;
const SVG_LARGE_SIZE: u16 = 48;

pub fn icon<'a>(handle: Handle) -> Svg<'a, Theme> {
    Svg::new(handle)
        .width(SVG_SIZE)
        .height(SVG_SIZE)
}

pub fn icon_large<'a>(handle: Handle) -> Svg<'a, Theme> {
    Svg::new(handle)
        .width(SVG_LARGE_SIZE)
        .height(SVG_LARGE_SIZE)
}

pub fn tooltiped_icon<'a>(handle: Handle, tooltip_text: String) -> Container<'a, Message> {
    Container::new(
    Tooltip::new(
        icon(handle)
            .width(SVG_SIZE)
            .height(SVG_SIZE),
        container(
            text(tooltip_text)
        )
            .padding(10)
            .style(container::rounded_box),
            Position::Top
    )
)
}