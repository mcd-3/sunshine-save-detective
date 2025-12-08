use iced::Length::Fill;
use iced::widget::Text;
use iced::{Alignment::Center, Font, alignment::Vertical::Bottom, color, widget::text};
use iced::font::Weight::Semibold;
use dark_light::Mode;

use crate::{
    models::{
        mem_card::MemoryCard,
        save_file::SaveFile
    },
    utils::data::get_blue_coin_shine_count
};

const FONT_SIZE: u16 = 18;
const TEXT_COLOUR_LIGHT_MODE: u32 = 0x000f88;
const TEXT_COLOUR_DARK_MODE: u32 = 0x97c1ff;

pub fn blue_coin_shines_counter<'a>(
    selected_save: &SaveFile,
    mem_card: &MemoryCard
) -> Text<'a> {
    let text_colour = match dark_light::detect() {
        Ok(Mode::Dark) => TEXT_COLOUR_DARK_MODE,
        Ok(Mode::Light) => TEXT_COLOUR_LIGHT_MODE,
        Ok(Mode::Unspecified) | Err(_) => TEXT_COLOUR_DARK_MODE,
    };

    text(
        format!(
            "Blue coin shines: {}",
            get_blue_coin_shine_count(&selected_save, &mem_card)
        )
    ).size(FONT_SIZE)
        .width(Fill)
        .height(Fill)
        .align_x(Center)
        .color(color!(text_colour))
        .font(
            Font {
                weight: Semibold,
                ..Font::DEFAULT
            }
        )
}