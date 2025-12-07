use std::time::Duration;

use iced::alignment::Horizontal::{Left, Right};
use iced::widget::{
    button,
    center,
    column,
    pick_list,
    radio,
    row,
    text,
    Row,
};
use iced::{
    window,
    Center,
    Element,
    Fill,
    Length,
    Task,
    Length::Shrink
};
use tokio::time::sleep;

use crate::{
    enums::{
        message::Message,
        file_error::FileError,
        world_identifier::WorldIdentifier
    },
    models::{
        mem_card::MemoryCard,
        save_file::SaveFile,
        world::World
    },
    ui::{
        components::{
            icon::{icon, icon_large},
            blue_coin_display::blue_coin_display,
            file_prompter::file_prompter,
            world_pick_list::world_pick_list,
            save_radio::save_radio
        }
    },
    utils::{
        data::{
            get_save_file,
            get_worlds_data_list,
            get_blue_coin_count,
            get_world_id_index,
        },
        file_pick::pick_file   
    }
};

// TODO: Follow good examples: https://github.com/fogarecious/iced_tutorial
const DEFAULT_SAVE_FILE_INDEX: u8 = SaveFile::SAVE_A_FILE_INDEX + 1;
const DEFAULT_SPACING: u16 = 15;
const TEXT_SIZE_HEADER: u16 = 18;
const TEXT_SIZE_SUBHEADER: u16 = 12;
const DEFAULT_ROW_HEIGHT: u16 = 36;

#[derive(Default)]
pub struct App {
    is_uploaded: bool,
    pub file_error: FileError,
    selected_save: SaveFile,
    mem_card: MemoryCard,
    pub worlds: Vec<World>,
    pub selected_world: WorldIdentifier,
    blue_coin_total: u16,
}

pub fn get_main_content(app: &App, is_uploaded: bool) -> Row<'_, Message> {
    if is_uploaded {
        let coin_hs_str: String = match app.selected_world {
            WorldIdentifier::CoronaId => String::from(" 0"),
            _ => {
                format!(" {}",
                    String::from(
                        &app.worlds[
                            get_world_id_index(app.selected_world)
                        ].get_coin_hs()
                            .to_string()
                    )
                )
            }
        };

        let bc_total_str: String = format!(" {}", &app.blue_coin_total.to_string());

        return row![
            column![
                row![
                    save_radio(app.selected_save.index),
                    row![
                        row![
                            icon("src/assets/sunshine.svg"),
                            text(
                                format!(
                                    " {}    ",
                                    app.selected_save.get_shine_count(&app.mem_card)
                                )
                            ),
                            icon("src/assets/blue_coin_collected.svg"),
                            text(bc_total_str),
                        ].width(Fill)
                            .align_y(Center),
                        column![
                            button("Upload")
                                .on_press(Message::FileSelectOpen)
                        ].width(Fill)
                            .spacing(DEFAULT_SPACING)
                            .align_x(Right)
                    ].width(Length::FillPortion(1))
                        .height(Fill)
                        .align_y(Center)
                ].spacing(DEFAULT_SPACING)
                    .height(DEFAULT_ROW_HEIGHT),
                row![
                    world_pick_list(app.selected_world),
                    row![
                        icon("src/assets/yellow_coin.svg"),
                        text(coin_hs_str)
                    ].width(Length::FillPortion(1))
                        .height(Fill)
                        .align_y(Center),
                ].spacing(DEFAULT_SPACING)
                    .height(DEFAULT_ROW_HEIGHT + 4),
                row![
                    blue_coin_display(app)
                ].spacing(DEFAULT_SPACING),
            ].height(Fill)
                .width(Fill)
                .align_x(Center),
        ].height(Fill)
            .align_y(Center)
    } else {
        return file_prompter(&app.file_error);
    }
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Default => window::get_latest().and_then(window::close),
            Message::FileSelectOpen => {
                let result: MemoryCard = pick_file();

                if matches!(result.file_error, FileError::NoError) {
                    self.mem_card = result.clone();
                    self.is_uploaded = true;
                    self.file_error = FileError::NoError;
                    self.selected_save = match Some(DEFAULT_SAVE_FILE_INDEX) {
                        Some(i) => get_save_file(i),
                        None => get_save_file(DEFAULT_SAVE_FILE_INDEX)
                    };
                    self.worlds = get_worlds_data_list(&self.mem_card, &self.selected_save);
                    self.selected_world = WorldIdentifier::DelfinoId;
                    self.blue_coin_total = get_blue_coin_count(&self.worlds);
                } else {
                    self.is_uploaded = false;
                    self.file_error = result.file_error;
                    return Task::perform(
                        sleep(
                            Duration::from_millis(50)
                        ),
                        move |_e| Message::DisplayFileError(result.file_error)
                    );
                }

                Task::none()
            },
            Message::DisplayFileError(e) => {
                self.file_error = e;
                self.is_uploaded = false;
                Task::none()
            }
            Message::SelectSaveFile(i) => {
                self.selected_save = get_save_file(i);
                self.worlds = get_worlds_data_list(
                    &self.mem_card,
                    &self.selected_save
                );
                self.selected_world = WorldIdentifier::DelfinoId;
                self.blue_coin_total = get_blue_coin_count(&self.worlds);
                Task::none()
            },
            Message::SelectWorld(id) => {
                self.selected_world = id;
                Task::none()
            }
            _ => {
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let version: String = String::from("Version: v.") + env!["CARGO_PKG_VERSION"];

        let header: Row<'_, Message> = row![
                icon_large("src/assets/sunshine_detective.svg"),
                text("  Sunshine Save Detective")
                    .size(TEXT_SIZE_HEADER)
                    .width(Length::FillPortion(1))
                    .height(Fill)
                    .align_x(Left)
                    .align_y(Center),
            ].height(Shrink);

        let footer: Row<'_, Message> = row![
                text("License")
                    .size(TEXT_SIZE_SUBHEADER)
                    .width(Length::FillPortion(1))
                    .align_x(Left),
                text(version.clone())
                    .size(TEXT_SIZE_SUBHEADER)
                    .width(Length::FillPortion(1))
                    .align_x(Right),
            ].height(Shrink);

        let content = column![
            header,
            get_main_content(&self, self.is_uploaded),
            footer,
        ].height(Fill)
            .spacing(10)
            .align_x(Center);

        center(content).padding(20).into()
    }
}
