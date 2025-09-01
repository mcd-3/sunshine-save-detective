use iced::alignment::Horizontal::{Left, Right};
use iced::widget::{button, center, column, row, text};
use iced::{window};
use iced::color;
use iced::{Center, Element, Task};

use rfd::FileDialog;

// #[derive(Default)]
// pub struct Exit {
//     show_confirm: bool,
// }

pub fn pick_file() {
    let files = FileDialog::new()
        .add_filter("Gamecube Saves", &["gci"])
        .set_directory("/")
        .pick_file();
}

#[derive(Default)]
pub struct Upload {
    show_file_dialog: bool,
    is_uploading: bool,
}

// #[derive(Debug, Clone, Copy)]
// pub enum Message {
//     Confirm,
//     Exit,
// }

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Default,
    FileSelect,
    Uploading
}

impl Upload {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Default => window::get_latest().and_then(window::close),
            _ => {
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let version: String = String::from("Version: v.") + env!["CARGO_PKG_VERSION"];

        let content = column![
            row![
                text("Sunshine Save Detective")
                    .size(24)
                    .color(color!(0xFFFFFF))
            ].width(iced::Length::Fill),
            row![
                column![
                    text("Select a GCI file")
                        .size(16)
                        .color(color!(0xFFFFFF)),
                    button("Upload")
                        .padding([10, 20])
                        .on_press({
                            pick_file();
                            Message::FileSelect
                        })
                ]
            ].width(iced::Length::Fill),
            row![
                text("License")
                    .size(12)
                    .color(color!(0xFFFFFF))
                    .align_x(Left),
                text(version)
                    .size(12)
                    .color(color!(0xAAAAAA))
                    .align_x(Right)
            ].width(iced::Length::Fill)
        ]
        .spacing(10)
        .align_x(Center);

        center(content).padding(20).into()
    }
}


// impl Exit {
//     pub fn update(&mut self, message: Message) -> Task<Message> {
//         match message {
//             Message::Confirm => window::get_latest().and_then(window::close),
//             Message::Exit => {
//                 self.show_confirm = true;

//                 Task::none()
//             }
//         }
//     }

//     pub fn view(&self) -> Element<'_, Message> {
//         let content = if self.show_confirm {
//             column![
//                 "Are you sure you want to exit?",
//                 button("Yes, exit now")
//                     .padding([10, 20])
//                     .on_press(Message::Confirm),
//             ]
//         } else {
//             column![
//                 "Click the button to exit",
//                 button("Exit").padding([10, 20]).on_press(Message::Exit),
//             ]
//         }
//         .spacing(10)
//         .align_x(Center);

//         center(content).padding(20).into()
//     }
// }