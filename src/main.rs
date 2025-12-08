mod models {
    pub mod blue_coin;
    pub mod mem_card;
    pub mod world;
    pub mod save_file;
    pub mod shine;
}

mod data {
    pub mod shine_data;
    pub mod blue_coin_data;
    pub mod coin_data;
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
    }
}

mod enums {
    pub mod message;
    pub mod file_error;
    pub mod world_identifier;
}

mod utils {
    pub mod file_pick;
    pub mod data;
    pub mod assets;
    pub mod string;
}

use iced::{application, Result, window::Settings, Size};
use ui::views::app::App;

fn main() -> Result {
    let app_name = env!("CARGO_PKG_NAME");

    application(&*app_name, App::update, App::view)
        .window_size(Size::new(800.0, 600.0))
        .window(Settings {
            min_size: Some(Size::new(800.0, 600.0)),
            ..Settings::default()
        })
        .run()
}
