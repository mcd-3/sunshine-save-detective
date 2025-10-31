use iced::alignment::Horizontal::{Left, Right};
use iced::alignment::Vertical::Bottom;
use iced::widget::{button, center, column, pick_list, radio, row, text, Image, Row};
use iced::Length::Shrink;
use iced::{window};
use iced::{Center, Element, Fill, Length, Task};

use crate::data::blue_coin_data::BlueCoinData;
use crate::data::coin_data::CoinData;
use crate::enums::file_error::FileError;
use crate::models::mem_card::MemoryCard;
use crate::models::save_file::SaveFile;
use crate::models::world::World;
use crate::utils::file_pick::pick_file;

// TODO: Follow good examples: https://github.com/fogarecious/iced_tutorial

pub fn get_main_content(app: &App, is_uploaded: bool) -> Row<'static, Message> {
    if is_uploaded {
        let save_a: SaveFile = SaveFile::new(
            SaveFile::SAVE_A_LABEL,
            SaveFile::SAVE_A_FILE_INDEX,
            SaveFile::SAVE_A_BC_START,
            SaveFile::SAVE_A_BC_END,
        );

        let w_delfino = World::new(
            &app.mem_card,
            &save_a,
            String::from("Delfino"),
            CoinData::get_delfino_coin_data(),
            BlueCoinData::get_delfino_bcs()
        );

        let result = [
            "Coin HS:",
            w_delfino
                .get_coin_hs()
                .to_string()
                .as_str()
        ].join(" ");

        return row![
            column![
                row![
                    row![
                        text("Save File: "),
                        radio("A ", 1, app.selected_save_file, |i| Message::SelectSaveFile(i)),
                        radio("B ", 2, app.selected_save_file, |i| Message::SelectSaveFile(i)),
                        radio("C", 3, app.selected_save_file, |i| Message::SelectSaveFile(i)),
                    ].width(Length::FillPortion(1)),
                    row![
                        text("Total Shines:  "),
                        text(result),
                    ].width(Length::FillPortion(1))
                ].spacing(15)
                    .height(32),
                row![
                    row![
                        text("World: "),
                        pick_list(
                            vec!["Construct from function"],
                            Some("Construct from function"),
                            |_| Message::DoNothing
                        ),
                    ].width(Length::FillPortion(1)),
                    row![
                        text("Coin Highscore: 999 "),
                    ].width(Length::FillPortion(1))
                ]
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

#[derive(Default)]
pub struct App {
    show_file_dialog: bool,
    is_uploaded: bool,
    selected_save_file: Option<u8>,
    mem_card: MemoryCard,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Default,
    FileSelectOpen,
    Uploading,
    SelectSaveFile(u8),
    DoNothing,
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
                    self.selected_save_file = Some(SaveFile::SAVE_A_FILE_INDEX + 1)
                }

                Task::none()
            },
            Message::SelectSaveFile(i) => {
                self.selected_save_file = Some(i);
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
