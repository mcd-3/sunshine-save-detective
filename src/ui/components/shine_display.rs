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
    Svg,
    Tooltip,
    container,
    text,
    tooltip::Position,
    column
};

use crate::models::world::World;
use crate::utils::data::get_shine_row_lists;
use crate::{
    enums::{
        message::Message,
        world_identifier::WorldIdentifier
    },
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

const SVG_SIZE: u16 = 40;
const SVG_SIZE_XL: u16 = 128;

pub fn shine_display(worlds: &Vec<World>, selected_world: WorldIdentifier) -> Column<'_, Message> {
    // Make corona larger since it's the final shine in-game
    let shine_size: u16 = match selected_world {
        WorldIdentifier::CoronaId => SVG_SIZE_XL,
        _ => SVG_SIZE
    };

    let row1_padding: f32 = match selected_world {
        WorldIdentifier::CoronaId => 0.0,
        _ => 12.0
    };

    let shine_row_lists = get_shine_row_lists(
        &worlds[
            get_world_id_index(selected_world)
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
                                .width(shine_size)
                                .height(shine_size),
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
            .padding(Padding::ZERO.bottom(row1_padding)),
        column![
            Row::with_children(r2)
                .width(Length::Fill)
        ].width(Length::Fill)
    ]
}