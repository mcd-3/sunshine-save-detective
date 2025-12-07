use iced::{
    Alignment::Center,
    Length::{self, Fill},
    widget::{Row, pick_list, row}
};

use crate::{
    enums::{
        message::Message,
        world_identifier::WorldIdentifier
    },
    models::world::World
};

pub fn world_pick_list<'a>(selected_world: WorldIdentifier) -> Row<'a, Message> {
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
            Some(World::get_name_from_identifier(selected_world)),
            |s| Message::SelectWorld(World::get_identifier_from_name(s))
        ),
    ].width(Length::FillPortion(1))
        .height(Fill)
        .align_y(Center)
}