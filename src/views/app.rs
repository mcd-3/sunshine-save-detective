use iced::alignment::Horizontal::{Left, Right};
use iced::alignment::Vertical::Bottom;
use iced::widget::{button, center, column, pick_list, radio, row, text, Image, Row};
use iced::Length::Shrink;
use iced::{window};
use iced::{Center, Element, Fill, Length, Task};

use rfd::FileDialog;

use crate::models::mem_card::MemoryCard;

// TODO: Follow good examples: https://github.com/fogarecious/iced_tutorial

// TODO: Change this return type to an enum
pub fn pick_file() -> bool {
    let result_path = FileDialog::new()
        .add_filter("Gamecube Saves", &["gci"])
        .set_directory("/")
        .pick_file();

    let path = match result_path {
        Some(pb) => pb.display().to_string(),
        None => String::from(""),
    };

    if path == "" {
        return false;
    }

    let mem_card: MemoryCard = MemoryCard::read_from_file(&path);

    println!("We got the save! {}", mem_card.get_mem_data()[0]);

    true
}

pub fn get_main_content(app: &App, is_uploaded: bool) -> Row<'static, Message> {
    if is_uploaded {
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
                        text("Total Shines: NA  "),
                        text("Total Blue Coins: 255"),
                    ].width(Length::FillPortion(1))
                ].spacing(15),
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
            ],
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
    selected_save_file: Option<u32>
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Default,
    FileSelectOpen,
    Uploading,
    SelectSaveFile(u32),
    DoNothing,
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Default => window::get_latest().and_then(window::close),
            Message::FileSelectOpen => {
                let result = pick_file();

                if result {
                    self.is_uploaded = true;
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
