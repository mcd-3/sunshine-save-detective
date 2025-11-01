use iced::alignment::Horizontal::{Left, Right};
use iced::widget::{button, center, column, pick_list, radio, row, text, Image, Row};
use iced::Length::Shrink;
use iced::{window};
use iced::{Center, Element, Fill, Length, Task};

use crate::enums::world_identifier::WorldIdentifier;
use crate::utils::data::get_world_id_index;
use crate::{
    enums::file_error::FileError,
    models::{
        mem_card::MemoryCard,
        save_file::SaveFile,
        world::World
    },
    utils::{
        data::{
            get_save_file,
            get_worlds_data_list
        },
        file_pick::pick_file   
    }
};

// TODO: Follow good examples: https://github.com/fogarecious/iced_tutorial
const DEFAULT_SAVE_FILE_INDEX: u8 = SaveFile::SAVE_A_FILE_INDEX + 1;

#[derive(Default)]
pub struct App {
    is_uploaded: bool,
    selected_save: SaveFile,
    mem_card: MemoryCard,
    worlds: Vec<World>,
    selected_world: WorldIdentifier,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Default,
    FileSelectOpen,
    SelectSaveFile(u8),
    SelectWorld(WorldIdentifier),
    DoNothing,
}

pub fn get_main_content(app: &App, is_uploaded: bool) -> Row<'static, Message> {
    if is_uploaded {
        let result = [
            "Coin Highscore:",
            &app.worlds[get_world_id_index(app.selected_world)]
                .get_coin_hs()
                .to_string()
                .as_str()
        ].join(" ");

        return row![
            column![
                row![
                    row![
                        text("Save File: "),
                        radio("A ", 1, Some(app.selected_save.index + 1), |i| Message::SelectSaveFile(i)),
                        radio("B ", 2, Some(app.selected_save.index + 1), |i| Message::SelectSaveFile(i)),
                        radio("C", 3, Some(app.selected_save.index + 1), |i| Message::SelectSaveFile(i)),
                    ].width(Length::FillPortion(1)),
                    row![
                        text("Total Shines: 120 "),
                        text("Total Blue Coins: 240"),
                    ].width(Length::FillPortion(1))
                ].spacing(15)
                    .height(32),
                row![
                    row![
                        text("World: "),
                        pick_list(
                            vec![
                                World::NAME_DELFINO,
                                World::NAME_BIANCO,
                                World::NAME_RICCO,
                                World::NAME_GELATO,
                                World::NAME_PINNA,
                                World::NAME_SIRENA,
                                World::NAME_NOKI,
                                World::NAME_PIANTA,
                                World::NAME_CORONA,
                            ],
                            Some(World::get_name_from_identifier(app.selected_world)),
                            |s| Message::SelectWorld(World::get_identifier_from_name(s))
                        ),
                    ].width(Length::FillPortion(1)),
                    row![
                        text(result),
                    ].width(Length::FillPortion(1))
                ].spacing(15)
            ].height(Fill)
                .width(Fill)
                .align_x(Center),
        ].height(Fill)
            .align_y(Center)
    } else {
        return row![
            column![
                text("Select a GCI file")
                    .size(22),
                button("Upload")
                    .padding([10, 20])
                    .on_press(Message::FileSelectOpen)
            ].align_x(Center)
                .spacing(15)
        ].height(Fill)
            .align_y(Center)
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
                    self.selected_save = match Some(DEFAULT_SAVE_FILE_INDEX) {
                        Some(i) => get_save_file(i),
                        None => get_save_file(DEFAULT_SAVE_FILE_INDEX)
                    };
                    self.worlds = get_worlds_data_list(&self.mem_card, &self.selected_save);
                    self.selected_world = WorldIdentifier::DelfinoId;
                }

                Task::none()
            },
            Message::SelectSaveFile(i) => {
                self.selected_save = get_save_file(i);
                self.worlds = get_worlds_data_list(
                    &self.mem_card,
                    &self.selected_save
                );
                self.selected_world = WorldIdentifier::DelfinoId;
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
                Image::new("src/assets/sun_icon.png").width(32).height(32),
                text(" Sunshine Save Detective")
                    .size(18)
                    .width(Length::FillPortion(1))
                    .height(Fill)
                    .align_x(Left)
                    .align_y(Center),
            ].height(Shrink);

        let footer: Row<'_, Message> = row![
                text("License")
                    .size(12)
                    .width(Length::FillPortion(1))
                    .align_x(Left),
                text(version.clone())
                    .size(12)
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
