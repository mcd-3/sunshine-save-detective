use iced::{
    Alignment::Center,
    Length::{Fill, FillPortion},
    widget::{
        Row,
        row,
        svg::Handle,
        text,
    }
};

use crate::{
    enums::message::Message,
    ui::components::icon::tooltiped_icon,
    sunshine_lib::utils::string::padded_string,
};

pub fn count_display<'a>(
    icon_handle: Handle,
    tooltip_text: &'a str,
    count: u16
) -> Row<'a, Message>{
    row![
        tooltiped_icon(
            icon_handle,
            String::from(tooltip_text)
        ),
        text(padded_string(count.to_string().as_str(), 1, 0)),
    ].width(FillPortion(1))
        .height(Fill)
        .align_y(Center)
}