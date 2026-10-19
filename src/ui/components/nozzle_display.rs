use iced::{
    Alignment::Center,
    Length::{self, Fill, Shrink},
    Padding,
    alignment::Horizontal::Right,
    widget::{Row, row}
};

use crate::{
    enums::{
        message::Message,
    },
    models::world::World,
    ui::components::icon::tooltiped_icon,
    utils::data::{
        get_svg_path_from_nozzle_collected,
        get_world_id_index
    },
    sunshine_lib::enums::{
        world_identifier::WorldIdentifier,
        nozzle_type::NozzleType
    },
};

pub fn nozzle_display(
    worlds: &Vec<World>,
    selected_world: WorldIdentifier
) -> Row<'_, Message> {
    return match selected_world {
        WorldIdentifier::SirenaId | WorldIdentifier::PinnaId => { row![] }, // Nothing
        WorldIdentifier::PiantaId | WorldIdentifier::CoronaId => {
            let nozzle_list = &worlds[
                get_world_id_index(selected_world)
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
            let nozzle_list = &worlds[
                get_world_id_index(selected_world)
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
}