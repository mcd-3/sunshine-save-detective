use iced::{
    Alignment::Center,
    Length::{Fill, FillPortion},
    widget::{
        Container,
        Row,
        Tooltip,
        container,
        row,
        svg::Handle,
        text,
        tooltip::Position
    }
};

use crate::{
    enums::message::Message,
    ui::components::icon::icon,
    utils::string::padded_string
};

pub fn count_display<'a>(
    icon_handle: Handle,
    tooltip_text: &'a str,
    count: u16
) -> Row<'a, Message>{
    row![
        Container::new(
            Tooltip::new(
                icon(icon_handle),
                container(
                    text(tooltip_text)
                )
                    .padding(10)
                    .style(container::rounded_box),
                    Position::Top
            )
        ),
        text(padded_string(count.to_string().as_str(), 1, 0)),
    ].width(FillPortion(1))
        .height(Fill)
        .align_y(Center)
}