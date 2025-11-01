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