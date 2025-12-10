use std::time::Duration;

use iced::Length::{FillPortion, Shrink};
use iced::alignment::Horizontal::Right;
use iced::widget::{Row, button, center, column, row};
use iced::{
    Center,
    Element,
    Fill,
    Length,
    Padding,
    Task,
    window,
};
use tokio::time::sleep;

use crate::enums::nozzle_type::NozzleType;
use crate::ui::components::icon::{tooltiped_icon};
use crate::utils::data::get_svg_path_from_nozzle_collected;
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
            blue_coin_display::blue_coin_display,
            file_prompter::file_prompter,
            world_pick_list::world_pick_list,
            save_radio::save_radio,
            footer::footer,
            navbar::navbar,
            shine_display::shine_display,
            blue_coin_shines_counter::blue_coin_shines_counter,
            count_display::count_display,
        }
    },
    utils::{
        data::{
            get_save_file,
            get_worlds_data_list,
            get_blue_coin_count,
            get_world_id_index,
        },
        file_pick::pick_file,
        assets::{
            blue_coin_collected_svg_path,
            sunshine_collected_svg_path,
            yellow_coin_svg_path,
            lives_svg_path,
        },
    }
};

// TODO: Follow good examples: https://github.com/fogarecious/iced_tutorial
const DEFAULT_SAVE_FILE_INDEX: u8 = SaveFile::SAVE_A_FILE_INDEX + 1;
const SMALL_SPACING: u16 = 12;
const DEFAULT_SPACING: u16 = 16;
const DEFAULT_PADDING: f32 = 25.0;
const DEFAULT_ROW_HEIGHT: u16 = 38;

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

pub fn get_main_content(app: &App, is_uploaded: bool) -> Row<'_, Message> {
    if is_uploaded {
        let shine_rows = match &app.selected_world {
            WorldIdentifier::DelfinoId => {
                row![
                    column![
                        shine_display(&app.worlds, app.selected_world),
                        row![
                            blue_coin_shines_counter(&app.selected_save, &app.mem_card).height(Fill)
                        ].padding(Padding::new(0.0).top(DEFAULT_PADDING))
                    ]
                ]
            },
            _ => {
                row![
                    shine_display(&app.worlds, app.selected_world),
                ].spacing(DEFAULT_SPACING)
            }
        };

        let shine_portion = match &app.selected_world {
            WorldIdentifier::DelfinoId => 3,
            _ => 2,
        };

        let nozzles: Row<'_, Message> = match &app.selected_world {
            WorldIdentifier::SirenaId | WorldIdentifier::PinnaId => { row![] }, // Null
            WorldIdentifier::PiantaId | WorldIdentifier::CoronaId => {
                let nozzle_list = &app.worlds[
                    get_world_id_index(app.selected_world)
                ].nozzles;
                row![
                    tooltiped_icon(
                        get_svg_path_from_nozzle_collected(
                            nozzle_list[0].get_is_collected(),
                            NozzleType::Rocket
                        ),
                        nozzle_list[0].get_overview()
                    ).width(Length::Fill)
                        .align_x(Right),
                ].height(Fill)
                    .width(Fill)
                    .align_y(Center)
            }, // Rocket only
            _ => {
                let nozzle_list = &app.worlds[
                    get_world_id_index(app.selected_world)
                ].nozzles;
                row![
                    tooltiped_icon(
                        get_svg_path_from_nozzle_collected(
                            nozzle_list[0].get_is_collected(),
                            NozzleType::Rocket
                        ),
                        nozzle_list[0].get_overview()
                    ).width(Length::Fill)
                        .align_x(Right),
                    tooltiped_icon(
                        get_svg_path_from_nozzle_collected(
                            nozzle_list[1].get_is_collected(),
                            NozzleType::Turbo
                        ),
                        nozzle_list[1].get_overview()
                    ).width(Shrink)
                        .padding(Padding::ZERO.left(12.0))
                        .align_x(Right)
                ].height(Fill)
                    .width(Fill)
                    .align_y(Center)
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
                            button("Upload")
                                .on_press(Message::FileSelectOpen)
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
                        nozzles,
                    ]
                ].spacing(DEFAULT_SPACING)
                    .height(DEFAULT_ROW_HEIGHT + 4),
                column![
                    shine_rows.spacing(SMALL_SPACING).height(FillPortion(shine_portion)),
                    row![
                        blue_coin_display(&app.worlds, app.selected_world)
                    ].height(FillPortion(2)),
                ]
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
        let content = column![
            navbar(),
            get_main_content(&self, self.is_uploaded),
            footer(),
        ].height(Fill)
            .spacing(10)
            .align_x(Center);

        center(content).padding(20).into()
    }
}
