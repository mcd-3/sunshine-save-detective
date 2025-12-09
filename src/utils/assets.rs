use iced::advanced::svg::Handle;

const SUNSHINE_DETECTIVE_SVG: &[u8; 14230] =
    include_bytes!("./../assets/sunshine_detective.svg");
const SUNSHINE_COLLECTED_SVG: &[u8; 11863] =
    include_bytes!("./../assets/sunshine.svg");
const SUNSHINE_MISSING_SVG: &[u8; 11894] =
    include_bytes!("./../assets/sunshine_missing.svg");
const BLUE_COIN_COLLECTED_SVG: &[u8; 2674] =
    include_bytes!("./../assets/blue_coin_collected.svg");
const BLUE_COIN_MISSING_SVG: &[u8; 2674] =
    include_bytes!("./../assets/blue_coin_missing.svg");
const YELLOW_COIN_SVG: &[u8; 2142] =
    include_bytes!("./../assets/yellow_coin.svg");

pub fn sunshine_collected_svg_path() -> Handle {
    Handle::from_memory(SUNSHINE_COLLECTED_SVG.as_slice())
}

pub fn sunshine_missing_svg_path() -> Handle {
    Handle::from_memory(SUNSHINE_MISSING_SVG.as_slice())
}

pub fn sunshine_detective_svg_path() -> Handle {
    Handle::from_memory(SUNSHINE_DETECTIVE_SVG.as_slice())
}

pub fn blue_coin_collected_svg_path() -> Handle {
    Handle::from_memory(BLUE_COIN_COLLECTED_SVG.as_slice())
}

pub fn blue_coin_missing_svg_path() -> Handle {
    Handle::from_memory(BLUE_COIN_MISSING_SVG.as_slice())
}

pub fn yellow_coin_svg_path() -> Handle {
    Handle::from_memory(YELLOW_COIN_SVG.as_slice())
}