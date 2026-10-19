use iced::Padding;
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
            header::header,
            icon::icon
        }
    },
    models::world::World,
    utils::{
        data::{
            get_world_id_index,
            get_svg_path_from_shine_collected,
            get_shine_row_lists
        }, 
    },
    sunshine_lib::enums::world_identifier::WorldIdentifier,
};

const SVG_SIZE: u16 = 48;
const SVG_SIZE_XL: u16 = 128;
const HEADER_PADDING_LEFT: f32 = 16.0;
const HEADER_PADDING_TOP_BOTTOM: f32 = 8.0;
const PADDING: u16 = 4;
const TOOLTIP_PADDING: u16 = 10;

pub fn shine_display(
    worlds: &Vec<World>,
    selected_world: WorldIdentifier
) -> Column<'_, Message> {
    // Make corona larger since it's the final shine in-game
    let shine_size: u16 = match selected_world {
        WorldIdentifier::CoronaId => SVG_SIZE_XL,
        _ => SVG_SIZE
    };

    let shine_row_lists = get_shine_row_lists(
        &worlds[
            get_world_id_index(selected_world)
        ]
    );
    let mut r1: Vec<Element<'_, Message, Theme, Renderer>> = vec![];
    let mut r2: Vec<Element<'_, Message, Theme, Renderer>> = vec![];
    let mut r3: Vec<Element<'_, Message, Theme, Renderer>> = vec![];
    let mut row_number = 0;

    for r in shine_row_lists {
        if !r.is_empty() {
            for s in r {
                let new_element = Element::new(
                    Container::new(
                        Tooltip::new(
                            icon(
                                    get_svg_path_from_shine_collected(
                                        s.get_is_collected(),
                                        s.get_is_secret()
                                    )
                                )
                                .width(shine_size)
                                .height(shine_size),
                            container(
                                text(
                                        String::from(s.get_overview())
                                    )
                                )
                                .padding(TOOLTIP_PADDING)
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
        header(String::from("Shines"))
            .padding(
                Padding::ZERO
                    .left(HEADER_PADDING_LEFT)
                    .top(HEADER_PADDING_TOP_BOTTOM)
            ),
        column![
            Row::with_children(r1)
                .width(Length::Fill)
        ].width(Length::Fill)
            .padding(PADDING),
        column![
            Row::with_children(r2)
                .width(Length::Fill)
        ].width(Length::Fill)
            .padding(PADDING),
        column![
            Row::with_children(r3)
                .width(Length::Fill)
        ].width(Length::Fill)
            .padding(PADDING),
    ]
}