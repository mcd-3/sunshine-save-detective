use iced::{Theme, widget::{Svg}, advanced::svg::Handle};

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