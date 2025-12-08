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

use crate::{
    enums::message::Message,
    ui::{
        components::{
            header::header
        },
    },
    utils::{
        data::{
            get_world_id_index,
            get_svg_path_from_blue_coin_collected,
            get_blue_coin_row_lists
        }, 
    },
    enums::world_identifier::WorldIdentifier,
    models::world::World,
};

const SVG_SIZE: u16 = 28;

pub fn blue_coin_display(worlds: &Vec<World>, selected_world: WorldIdentifier) -> Column<'_, Message> {
    let blue_coin_row_lists = get_blue_coin_row_lists(
        &worlds[
            get_world_id_index(selected_world)
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
    };

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
}