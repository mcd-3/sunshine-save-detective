use iced::{Theme, widget::Svg};

const SVG_SIZE: u16 = 28;

pub fn icon(path: &str) -> Svg<'_, Theme> {
    Svg::from_path(path)
        .width(SVG_SIZE)
        .height(SVG_SIZE)
}