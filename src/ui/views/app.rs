use std::time::Duration;

use iced::Length::{FillPortion, Shrink};
use iced::alignment::Horizontal::Right;
use iced::alignment::Vertical::{Bottom, Top};
use iced::widget::{Row, text, center, column, container, row};
use iced::{
    Border, Center, Color, Element, Fill, Length, Padding, Task
};
use tokio::time::sleep;

use crate::{
    enums::{
        message::Message,
    },
    models::{
        mem_card::MemoryCard,
        save_file::SaveFile,
        world::World
    },
    ui::{
        components::{
            blue_coin_display::blue_coin_display,
            file_prompter::file_prompter,
            world_pick_list::world_pick_list,
            save_radio::save_radio,
            footer::footer,
            navbar::navbar,
            shine_display::shine_display,
            blue_coin_shines_counter::blue_coin_shines_counter,
            count_display::count_display,
            nozzle_display::nozzle_display,
        }
    },
    utils::{
        data::{
            get_save_file,
            get_worlds_data_list,
            get_blue_coin_count,
            get_world_id_index,
        },
        file_pick::{
            pick_file,
            pick_directory,
        },
        assets::{
            blue_coin_collected_svg_path,
            sunshine_collected_svg_path,
            yellow_coin_svg_path,
            lives_svg_path,
        },
        report::write_report,
    },
    sunshine_lib::enums::{
        file_error::FileError,
        region::Region,
        world_identifier::WorldIdentifier,
    },
};

const DEFAULT_SAVE_FILE_INDEX: u8 = SaveFile::SAVE_A_FILE_INDEX + 1;
const DEFAULT_SPACING: u16 = 16;
const DEFAULT_PADDING: f32 = 12.0;
const DEFAULT_ROW_HEIGHT: u16 = 38;
const SHINES_ROW_DELFINO_HEIGHT: u16 = 148;
const PADDING_EXTRA: u16 = 16;
const BORDER_COLOUR: Color = Color::from_rgb(0.45, 0.45, 0.45);
const BORDER_WIDTH: f32 = 1.2;
const BORDER_RADIUS: f32 = 4.0;

#[derive(Default)]
pub struct App {
    is_uploaded: bool,
    file_error: FileError,
    selected_save: SaveFile,
    mem_card: MemoryCard,
    worlds: Vec<World>,
    selected_world: WorldIdentifier,
    blue_coin_total: u16,
    region: Region
}

pub fn get_main_content(app: &App, is_uploaded: bool) -> Row<'_, Message> {
    if is_uploaded {
        let shine_rows = match &app.selected_world {
            WorldIdentifier::DelfinoId => {
                row![
                    column![
                        shine_display(&app.worlds, app.selected_world)
                            .height(SHINES_ROW_DELFINO_HEIGHT),
                        row![
                            blue_coin_shines_counter(&app.selected_save, &app.mem_card)
                        ].padding(Padding::ZERO.top(DEFAULT_PADDING))
                    ].height(Fill)
                ]
            },
            _ => {
                row![
                    shine_display(&app.worlds, app.selected_world),
                ].height(Shrink)
            }
        };

        return row![
            column![
                row![
                    save_radio(app.selected_save.index),
                    row![
                        row![
                            count_display(
                                sunshine_collected_svg_path(),
                                "Total shines collected",
                                app.selected_save.get_shine_count(&app.mem_card)
                            ),
                            count_display(
                                blue_coin_collected_svg_path(),
                                "Total blue coins collected",
                                *&app.blue_coin_total
                            ),
                            count_display(
                                lives_svg_path(),
                                "Total lives",
                                *&app.selected_save.get_lives_count(&app.mem_card)
                            ),
                        ].width(FillPortion(3))
                            .height(Fill)
                            .align_y(Center),
                        column![
                            text(app.mem_card.get_region_name())
                                .height(Fill)
                                .align_y(Center)
                        ].width(FillPortion(1))
                            .spacing(DEFAULT_SPACING)
                            .align_x(Right)
                            .height(Fill)
                    ].width(Length::FillPortion(1))
                        .height(Fill)
                        .align_y(Center)
                ].spacing(DEFAULT_SPACING)
                    .height(DEFAULT_ROW_HEIGHT),
                row![
                    world_pick_list(app.selected_world),
                    row![
                        count_display(
                            yellow_coin_svg_path(),
                            "Coin highscore",
                            match app.selected_world {
                                WorldIdentifier::CoronaId => 0,
                                _ => {
                                    *&app.worlds[
                                        get_world_id_index(app.selected_world)
                                    ].get_coin_hs()
                                }
                            }
                        ),
                        nozzle_display(&app.worlds, app.selected_world),
                    ]
                ].spacing(DEFAULT_SPACING)
                    .height(DEFAULT_ROW_HEIGHT + 4)
                    .padding(Padding::ZERO.bottom(PADDING_EXTRA)),
                container(
                    column![
                        shine_rows.align_y(Top),
                        row![
                            blue_coin_display(&app.worlds, app.selected_world)
                        ].height(Shrink).align_y(Bottom).padding(Padding::ZERO.bottom(8)),
                    ]
                ).style(|_theme| {
                    container::Style {
                        border: Border {
                            width: BORDER_WIDTH,
                            color: BORDER_COLOUR,
                            radius: BORDER_RADIUS.into(),
                        },
                        ..Default::default()
                    }
                }),
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
                    self.region = self.mem_card.region
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
            },
            Message::FileReportWriteToOpen => {
                let path_buf = pick_directory();
                match path_buf {
                    Some(path) => {
                        let path_str = match path.as_path().to_str() {
                            Some(str) => String::from(str),
                            None => String::new(),
                        };

                        // If cancelled, don't write anything
                        if path_str.is_empty() {
                            return Task::none();
                        }

                        write_report(
                            &self.mem_card,
                            vec!['A', 'B', 'C'],
                            true,
                            true,
                            true,
                            true,
                            true,
                            true,
                            true,
                            true,
                            Some(path_str)
                        );
                    },
                    None => {}
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = column![
            navbar(!self.is_uploaded),
            get_main_content(&self, self.is_uploaded),
            footer(),
        ].height(Fill)
            .spacing(10)
            .align_x(Center);

        center(content).padding(20).into()
    }
}
