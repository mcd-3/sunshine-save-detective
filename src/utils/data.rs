use iced::widget::svg::Handle;

use crate::{
    utils::assets::{
        blue_coin_collected_svg_path,
        blue_coin_missing_svg_path,
        rocket_nozzle_collected_svg_path,
        rocket_nozzle_missing_svg_path,
        sunshine_collected_svg_path,
        sunshine_missing_svg_path,
        sunshine_secret_collected_svg_path,
        sunshine_secret_missing_svg_path,
        turbo_nozzle_collected_svg_path,
        turbo_nozzle_missing_svg_path,
    },
    sunshine_lib::{
        enums::nozzle_type::NozzleType,
    }
};

/// Gets the SVG asset corresponding to blue coin collected state
/// 
/// Returns a path string to the SVG
pub fn get_svg_path_from_blue_coin_collected(is_collected: bool) -> Handle {
    return if is_collected {
        blue_coin_collected_svg_path()
    } else {
        blue_coin_missing_svg_path()
    }
}

/// Gets the SVG asset corresponding to blue coin collected state
/// 
/// Returns a path string to the SVG
pub fn get_svg_path_from_shine_collected(
    is_collected: bool,
    is_secret: bool
) -> Handle {
    return if is_collected {
        if is_secret {
            sunshine_secret_collected_svg_path()
        } else {
            sunshine_collected_svg_path()
        }
    } else {
        if is_secret {
            sunshine_secret_missing_svg_path()
        } else {
            sunshine_missing_svg_path()
        }
    }
}

/// Gets the SVG asset corresponding to blue coin collected state
/// 
/// Returns a path string to the SVG
pub fn get_svg_path_from_nozzle_collected(
    is_collected: bool,
    nozzle_type: NozzleType
) -> Handle {
    if is_collected {
        return match nozzle_type {
            NozzleType::Rocket => rocket_nozzle_collected_svg_path(),
            NozzleType::Turbo => turbo_nozzle_collected_svg_path()
        }
    } else {
        return match nozzle_type {
            NozzleType::Rocket => rocket_nozzle_missing_svg_path(),
            NozzleType::Turbo => turbo_nozzle_missing_svg_path()
        }
    }
}