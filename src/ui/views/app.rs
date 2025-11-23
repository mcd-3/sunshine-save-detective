use std::time::Duration;

use iced::alignment::Horizontal::{Left, Right};
use iced::widget::tooltip::Position;
use iced::widget::{container, Container, Tooltip};
use iced::widget::{
    button,
    center,
    column,
    pick_list,
    radio,
    row,
    text,
    Image,
    Row,
    Svg,
    Text
};
use iced::Length::Shrink;
use iced::{window, Color, Renderer, Theme};
use iced::{Center, Element, Fill, Length, Task};
use tokio::time::sleep;

use crate::{
    enums::{
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
            header::header,
            icon::icon
        }
    },
    utils::{
        data::{
            get_save_file,
            get_worlds_data_list,
            get_blue_coin_count,
            get_world_id_index,
            get_svg_path_from_blue_coin_collected,
            get_blue_coin_row_lists
        },
        file_pick::pick_file   
    }
};

// TODO: Follow good examples: https://github.com/fogarecious/iced_tutorial
const DEFAULT_SAVE_FILE_INDEX: u8 = SaveFile::SAVE_A_FILE_INDEX + 1;
const SVG_SIZE: u16 = 28;
const DEFAULT_SPACING: u16 = 15;
const TEXT_SIZE_HEADER: u16 = 18;
const TEXT_SIZE_SUBHEADER: u16 = 12;

#[derive(Default)]
pub struct App {
    is_uploaded: bool,
    file_error: FileError,
    selected_save: SaveFile,
    mem_card: MemoryCard,
    worlds: Vec<World>,
    selected_world: WorldIdentifier,
    blue_coin_total: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Default,
    FileSelectOpen,
    DisplayFileError(FileError),
    SelectSaveFile(u8),
    SelectWorld(WorldIdentifier),
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

        let blue_coin_row_lists = get_blue_coin_row_lists(
            &app.worlds[
                get_world_id_index(app.selected_world)
            ]
        );
        let mut r1: Vec<Element<'_, Message, Theme, Renderer>> = vec![];
        let mut r2: Vec<Element<'_, Message, Theme, Renderer>> = vec![];
        let mut r3: Vec<Element<'_, Message, Theme, Renderer>> = vec![];
        let mut row_number = 0;

        for r in blue_coin_row_lists {
            if !r.is_empty() {
                for b in r {
                    let new_element = Element::new(
                        Container::new(
                            Tooltip::new(
                                Svg::from_path(
                                        get_svg_path_from_blue_coin_collected(
                                            b.get_is_collected()
                                        )
                                    )
                                    .width(SVG_SIZE)
                                    .height(SVG_SIZE),
                                container(
                                    text(
                                            b.get_overview()
                                        )
                                    )
                                    .padding(10)
                                    .style(container::rounded_box),
                                    Position::Top
                            )
                        ).width(Length::FillPortion(1))
                            .align_x(Center)
                    );

                    match row_number {
                        0 => r1.push(new_element),
                        1 => r2.push(new_element),
                        2 => r3.push(new_element),
                        _ => {}
                    }
                }
            }
            row_number = row_number + 1;
        }

        return row![
            column![
                row![
                    row![
                        text("Save File: "),
                        radio(
                            format!("{} ", SaveFile::SAVE_A_LABEL),
                            DEFAULT_SAVE_FILE_INDEX,
                            Some(app.selected_save.index + 1),
                            |i| Message::SelectSaveFile(i)
                        ),
                        radio(
                            format!("{} ", SaveFile::SAVE_B_LABEL),
                            DEFAULT_SAVE_FILE_INDEX + 1,
                            Some(app.selected_save.index + 1),
                            |i| Message::SelectSaveFile(i)
                        ),
                        radio(
                            SaveFile::SAVE_C_LABEL,
                            DEFAULT_SAVE_FILE_INDEX + 2,
                            Some(app.selected_save.index + 1),
                            |i| Message::SelectSaveFile(i)
                        ),
                    ].width(Length::FillPortion(1))
                        .height(32)
                        .align_y(Center),
                    row![
                        icon("src/assets/yellow_coin.svg"),
                        text(" 120 "),
                        icon("src/assets/blue_coin_collected.svg"),
                        text(bc_total_str),
                    ].width(Length::FillPortion(1))
                        .height(Fill)
                        .align_y(Center)
                ].spacing(DEFAULT_SPACING)
                    .height(36),
                row![
                    row![
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
                        icon("src/assets/yellow_coin.svg"),
                        text(coin_hs_str)
                    ].width(Length::FillPortion(1))
                ].spacing(DEFAULT_SPACING),
                row![
                    column![
                        header(String::from("Blue Coins")),
                        column![
                            Row::with_children(r1)
                                .width(Length::Fill)
                        ].width(Length::Fill)
                            .padding(8),
                        column![
                            Row::with_children(r2)
                                .width(Length::Fill)
                        ].width(Length::Fill)
                            .padding(8),
                        column![
                            Row::with_children(r3)
                                .width(Length::Fill)
                        ].width(Length::Fill)
                            .padding(8),
                    ]
                ].spacing(DEFAULT_SPACING),
            ].height(Fill)
                .width(Fill)
                .align_x(Center),
        ].height(Fill)
            .align_y(Center)
    } else {

        let error_text: &str = match app.file_error {
            FileError::FileRead => "Cannot read file",
            FileError::NotProperGame => "Game is not Super Mario Sunshine",
            FileError::NotProperRegion => "Game is not from NA region",
            FileError::NotValidSave => "Not a valid GCI file",
            _ => ""
        };
        let error_text_widget: Text = text(error_text)
            .color(
                Color::from_rgb(255.0, 0.0, 0.0)
            );

        return row![
            column![
                text("Select a GCI file")
                    .size(22),
                button("Upload")
                    .padding([10, 20])
                    .on_press(Message::FileSelectOpen),
                error_text_widget,
            ].align_x(Center)
                .spacing(DEFAULT_SPACING)
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
                            Duration::from_secs(1)
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
                Image::new("src/assets/sun_icon.png").width(32).height(32),
                text(" Sunshine Save Detective")
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
