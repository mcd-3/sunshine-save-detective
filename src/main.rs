#![windows_subsystem = "windows"]

mod models {
    pub mod blue_coin;
    pub mod mem_card;
    pub mod world;
    pub mod save_file;
    pub mod shine;
    pub mod nozzle;
}

mod data {
    pub mod shine_data;
    pub mod blue_coin_data;
    pub mod coin_data;
    pub mod nozzle_data;
}

mod ui {
    pub mod views {
        pub mod app;
    }
    pub mod components {
        pub mod header;
        pub mod icon;
        pub mod blue_coin_display;
        pub mod shine_display;
        pub mod error_warning;
        pub mod file_prompter;
        pub mod save_radio;
        pub mod world_pick_list;
        pub mod footer;
        pub mod navbar;
        pub mod blue_coin_shines_counter;
        pub mod count_display;
        pub mod nozzle_display;
    }
}

mod enums {
    pub mod message;
    pub mod file_error;
    pub mod world_identifier;
    pub mod nozzle_type;
}

mod utils {
    pub mod file_pick;
    pub mod data;
    pub mod assets;
    pub mod string;
}

use std::env;

use dark_light::Mode;
use iced::{
    Color,
    Result,
    Size,
    advanced::graphics::image::image_rs::ImageFormat,
    application,
    daemon::Appearance,
    window::{self, Settings}
};

use ui::views::app::App;

use crate::utils::assets::knewave_font_path;

const WINDOW_SIZE: Size = Size::new(800.0, 600.0);
const BACKGROUND_LIGHT_MODE: Color = Color::from_rgb(0.980, 0.980, 0.980);
const BACKGROUND_DARK_MODE: Color = Color::from_rgb(0.1, 0.1, 0.1);

fn main() -> Result {
    #[cfg(target_os = "linux")]
    {
        unsafe {
            // On Linux, the window is scaled wayyy too big
            env::set_var("WINIT_X11_SCALE_FACTOR", "1.0");
        };
    }

    let is_dark: &bool = match dark_light::detect() {
        Ok(Mode::Dark) => &true,
        Ok(Mode::Light) => &false,
        Ok(Mode::Unspecified) | Err(_) => &true,
    };

    let icon = window::icon::from_file_data(
        include_bytes!("./assets/logo.ico"),
        Some(ImageFormat::Ico)
    ).expect("Could not find logo.ico");

    application(
        "Sunshine Save Detective",
        App::update, App::view
    ).window(Settings {
        icon: Some(icon),
        min_size: Some(WINDOW_SIZE),
        size: WINDOW_SIZE,
        ..Settings::default()
    }).style(|_state, _theme| {
        Appearance {
            background_color: if *is_dark {
                BACKGROUND_DARK_MODE
            } else {
                BACKGROUND_LIGHT_MODE
            },
            text_color: if *is_dark { Color::WHITE } else { Color::BLACK },
        }
    }).font(knewave_font_path())
        .run()
}
