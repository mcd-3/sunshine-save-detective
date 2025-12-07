use iced::{
    Alignment::Center,
    Element,
    Length,
    Renderer,
    Theme
};
use iced::widget::{
    Column,
    Container,
    Row,
    Svg,
    Tooltip,
    container,
    text,
    tooltip::Position,
    column
};

use crate::utils::data::get_shine_row_lists;
use crate::{
    enums::message::Message,
    ui::{
        components::{
            header::header
        },
        views::{
            app::App
        }
    },
    utils::{
        data::{
            get_world_id_index,
            get_svg_path_from_shine_collected
        }, 
    }
};

const SVG_SIZE: u16 = 28;

pub fn shine_display(app: &App) -> Column<'_, Message> {
    let shine_row_lists = get_shine_row_lists(
        &app.worlds[
            get_world_id_index(app.selected_world)
        ]
    );
    let mut r1: Vec<Element<'_, Message, Theme, Renderer>> = vec![];
    let mut r2: Vec<Element<'_, Message, Theme, Renderer>> = vec![];
    let mut row_number = 0;

    for r in shine_row_lists {
        if !r.is_empty() {
            for s in r {
                let new_element = Element::new(
                    Container::new(
                        Tooltip::new(
                            Svg::from_path(
                                    get_svg_path_from_shine_collected(
                                        s.get_is_collected()
                                    )
                                )
                                .width(SVG_SIZE)
                                .height(SVG_SIZE),
                            container(
                                text(
                                        String::from(s.get_description())
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
                    _ => {}
                }
            }
        }
        row_number = row_number + 1;
    };

    column![
        header(String::from("Shines")),
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
    ]
}