use iced::alignment::Horizontal::{Left, Right};
use iced::alignment::Vertical::Bottom;
use iced::widget::{Image, button, center, column, row, text};
use iced::Length::Shrink;
use iced::{window};
use iced::{Center, Element, Fill, Length, Task};

use rfd::FileDialog;

// TODO: Follow good examples: https://github.com/fogarecious/iced_tutorial

pub fn pick_file() {
    let files = FileDialog::new()
        .add_filter("Gamecube Saves", &["gci"])
        .set_directory("/")
        .pick_file();
}

#[derive(Default)]
pub struct App {
    show_file_dialog: bool,
    is_uploading: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Default,
    FileSelectOpen,
    Uploading
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Default => window::get_latest().and_then(window::close),
            Message::FileSelectOpen => {
                pick_file();
                Task::none()
            }
            _ => {
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let version: String = String::from("Version: v.") + env!["CARGO_PKG_VERSION"];

        let content = column![
            row![
                Image::new("src/assets/sun_icon.png").width(32).height(32),
                text(" Sunshine Save Detective")
                    .size(18)
                    .width(Length::FillPortion(1))
                    .height(Fill)
                    .align_x(Left)
                    .align_y(Center),
            ].height(Shrink),
            row![
                column![
                    text("Select a GCI file")
                        .size(22),
                    button("Upload")
                        .padding([10, 20])
                        .on_press(Message::FileSelectOpen)
                ].align_x(Center)
                    .spacing(15)
            ].height(Fill)
                .align_y(Center),
            row![
                text("License")
                    .size(12)
                    .width(Length::FillPortion(1))
                    .align_x(Left),
                text(version)
                    .size(12)
                    .width(Length::FillPortion(1))
                    .align_x(Right),
            ].height(Shrink)
        ].height(Fill)
            .spacing(10)
            .align_x(Center);

        center(content).padding(20).into()
    }
}
