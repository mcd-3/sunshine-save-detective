use iced::{Theme, widget::Svg};

const SVG_SIZE: u16 = 28;
const SVG_LARGE_SIZE: u16 = 48;

pub fn icon(path: &str) -> Svg<'_, Theme> {
    Svg::from_path(path)
        .width(SVG_SIZE)
        .height(SVG_SIZE)
}

pub fn icon_large(path: &str) -> Svg<'_, Theme> {
    Svg::from_path(path)
        .width(SVG_LARGE_SIZE)
        .height(SVG_LARGE_SIZE)
}