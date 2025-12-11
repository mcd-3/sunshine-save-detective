use iced::advanced::svg::Handle;

const SUNSHINE_DETECTIVE_SVG: &[u8] =
    include_bytes!("./../assets/sunshine_detective.svg");
const SUNSHINE_COLLECTED_SVG: &[u8] =
    include_bytes!("./../assets/sunshine.svg");
const SUNSHINE_MISSING_SVG: &[u8] =
    include_bytes!("./../assets/sunshine_missing.svg");
const SUNSHINE_SECRET_COLLECTED_SVG: &[u8] =
    include_bytes!("./../assets/sunshine_secret_collected.svg");
const SUNSHINE_SECRET_MISSING_SVG: &[u8] =
    include_bytes!("./../assets/sunshine_secret_missing.svg");
const BLUE_COIN_COLLECTED_SVG: &[u8] =
    include_bytes!("./../assets/blue_coin_collected.svg");
const BLUE_COIN_MISSING_SVG: &[u8] =
    include_bytes!("./../assets/blue_coin_missing.svg");
const ROCKET_COLLECTED_SVG: &[u8] =
    include_bytes!("./../assets/rocket_collected.svg");
const ROCKET_MISSING_SVG: &[u8] =
    include_bytes!("./../assets/rocket_missing.svg");
const TURBO_COLLECTED_SVG: &[u8] =
    include_bytes!("./../assets/turbo_collected.svg");
const TURBO_MISSING_SVG: &[u8] =
    include_bytes!("./../assets/turbo_missing.svg");
const YELLOW_COIN_SVG: &[u8] =
    include_bytes!("./../assets/yellow_coin.svg");
const LIVES_PNG: &[u8] =
    include_bytes!("./../assets/lives.svg");

const KNEWAVE_FONT: &[u8; 56996] = include_bytes!("./../assets/fonts/knewave.ttf");

pub fn sunshine_collected_svg_path() -> Handle {
    Handle::from_memory(SUNSHINE_COLLECTED_SVG)
}

pub fn sunshine_missing_svg_path() -> Handle {
    Handle::from_memory(SUNSHINE_MISSING_SVG)
}

pub fn sunshine_secret_collected_svg_path() -> Handle {
    Handle::from_memory(SUNSHINE_SECRET_COLLECTED_SVG)
}

pub fn sunshine_secret_missing_svg_path() -> Handle {
    Handle::from_memory(SUNSHINE_SECRET_MISSING_SVG)
}

pub fn sunshine_detective_svg_path() -> Handle {
    Handle::from_memory(SUNSHINE_DETECTIVE_SVG)
}

pub fn blue_coin_collected_svg_path() -> Handle {
    Handle::from_memory(BLUE_COIN_COLLECTED_SVG)
}

pub fn blue_coin_missing_svg_path() -> Handle {
    Handle::from_memory(BLUE_COIN_MISSING_SVG)
}

pub fn rocket_nozzle_collected_svg_path() -> Handle {
    Handle::from_memory(ROCKET_COLLECTED_SVG)
}

pub fn rocket_nozzle_missing_svg_path() -> Handle {
    Handle::from_memory(ROCKET_MISSING_SVG)
}

pub fn turbo_nozzle_collected_svg_path() -> Handle {
    Handle::from_memory(TURBO_COLLECTED_SVG)
}

pub fn turbo_nozzle_missing_svg_path() -> Handle {
    Handle::from_memory(TURBO_MISSING_SVG)
}

pub fn yellow_coin_svg_path() -> Handle {
    Handle::from_memory(YELLOW_COIN_SVG)
}

pub fn lives_svg_path() -> Handle {
    Handle::from_memory(LIVES_PNG)
}

pub fn knewave_font_path() -> &'static [u8] {
    KNEWAVE_FONT
}