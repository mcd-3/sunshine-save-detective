use crate::{
    data::{
        blue_coin_data::BlueCoinData,
        coin_data::CoinData
    },
    enums::world_identifier::WorldIdentifier,
    models::{
        mem_card::MemoryCard,
        save_file::SaveFile,
        world::World
    }
};

/// Gets a save file from a save file option number
/// 
/// Returns a save file
pub fn get_save_file(save_file_option: u8) -> SaveFile {
    let save_index: u8 = save_file_option - 1;
    match save_index {
        SaveFile::SAVE_A_FILE_INDEX => {
            SaveFile::new_by_label(SaveFile::SAVE_A_LABEL)
        },
        SaveFile::SAVE_B_FILE_INDEX => {
            SaveFile::new_by_label(SaveFile::SAVE_B_LABEL)
        },
        SaveFile::SAVE_C_FILE_INDEX => {
            SaveFile::new_by_label(SaveFile::SAVE_C_LABEL)
        },
        _ => {
            SaveFile::new_by_label(SaveFile::SAVE_A_LABEL)
        },
    }
}

/// Gets a world identifier index from a world identifier
/// 
/// Returns a number
pub fn get_world_id_index(world_id: WorldIdentifier) -> usize {
    match world_id {
        WorldIdentifier::DelfinoId => 0,
        WorldIdentifier::BiancoId => 1,
        WorldIdentifier::RiccoId => 2,
        WorldIdentifier::GelatoId => 3,
        WorldIdentifier::PinnaId => 4,
        WorldIdentifier::SirenaId => 5,
        WorldIdentifier::NokiId => 6,
        WorldIdentifier::PiantaId => 7,
        WorldIdentifier::CoronaId => 8,
    }
}

/// Gets a list of all worlds and associated save data
/// 
/// Returns a vec of worlds
pub fn get_worlds_data_list(
    mem_card: &MemoryCard,
    save_file: &SaveFile
) -> Vec<World> {
    vec![
        World::new(
            mem_card,
            save_file,
            String::from(World::NAME_DELFINO),
            WorldIdentifier::DelfinoId,
            CoinData::get_delfino_coin_data(),
            BlueCoinData::get_delfino_bcs()
        ),
        World::new(
            mem_card,
            save_file,
            String::from(World::NAME_BIANCO),
            WorldIdentifier::BiancoId,
            CoinData::get_bianco_coin_data(),
            BlueCoinData::get_bianco_bcs()
        ),
        World::new(
            mem_card,
            save_file,
            String::from(World::NAME_RICCO),
            WorldIdentifier::RiccoId,
            CoinData::get_ricco_coin_data(),
            BlueCoinData::get_ricco_bcs()
        ),        
        World::new(
            mem_card,
            save_file,
            String::from(World::NAME_GELATO),
            WorldIdentifier::GelatoId,
            CoinData::get_gelato_coin_data(),
            BlueCoinData::get_gelato_bcs()
        ),
        World::new(
            mem_card,
            save_file,
            String::from(World::NAME_PINNA),
            WorldIdentifier::PinnaId,
            CoinData::get_pinna_coin_data(),
            BlueCoinData::get_pinna_bcs()
        ),
        World::new(
            mem_card,
            save_file,
            String::from(World::NAME_SIRENA),
            WorldIdentifier::SirenaId,
            CoinData::get_sirena_coin_data(),
            BlueCoinData::get_sirena_bcs()
        ),
        World::new(
            mem_card,
            save_file,
            String::from(World::NAME_NOKI),
            WorldIdentifier::NokiId,
            CoinData::get_noki_coin_data(),
            BlueCoinData::get_noki_bcs()
        ),
        World::new(
            mem_card,
            save_file,
            String::from(World::NAME_PIANTA),
            WorldIdentifier::PiantaId,
            CoinData::get_pianta_coin_data(),
            BlueCoinData::get_pianta_bcs()
        ),
        World::new(
            mem_card,
            save_file,
            String::from(World::NAME_CORONA),
            WorldIdentifier::CoronaId,
            CoinData::get_corona_coin_data(),
            BlueCoinData::get_corona_bcs()
        ),
    ]
}

/// Gets the total blue coin count for a list of worlds
/// 
/// Returns u16 count of blue coin total
pub fn get_blue_coin_count(world_list: &Vec<World>) -> u16 {
    let mut blue_coin_count: u16 = 0;

    for w in world_list {
        for b in &w.blue_coins {
            if b.get_is_collected() {
                blue_coin_count += 1;
            }
        }
    }

    blue_coin_count
}