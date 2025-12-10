use iced::widget::svg::Handle;

use crate::{
    data::{
        blue_coin_data::BlueCoinData, coin_data::CoinData, nozzle_data::NozzleData, shine_data::ShineData
    },
    enums::{
        nozzle_type::NozzleType,
        world_identifier::WorldIdentifier
    },
    models::{
        blue_coin::BlueCoin,
        mem_card::MemoryCard,
        save_file::SaveFile,
        shine::Shine,
        world::World
    },
    utils::assets::{
        blue_coin_collected_svg_path,
        blue_coin_missing_svg_path,
        rocket_nozzle_collected_svg_path,
        rocket_nozzle_missing_svg_path,
        sunshine_collected_svg_path,
        sunshine_missing_svg_path,
        turbo_nozzle_collected_svg_path,
        turbo_nozzle_missing_svg_path,
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
            BlueCoinData::get_delfino_bcs(),
            ShineData::get_delfino_shines(),
            NozzleData::get_delfino_nozzles(),
        ),
        World::new(
            mem_card,
            save_file,
            String::from(World::NAME_BIANCO),
            WorldIdentifier::BiancoId,
            CoinData::get_bianco_coin_data(),
            BlueCoinData::get_bianco_bcs(),
            ShineData::get_bianco_shines(),
            NozzleData::get_bianco_nozzles(),
        ),
        World::new(
            mem_card,
            save_file,
            String::from(World::NAME_RICCO),
            WorldIdentifier::RiccoId,
            CoinData::get_ricco_coin_data(),
            BlueCoinData::get_ricco_bcs(),
            ShineData::get_ricco_shines(),
            NozzleData::get_ricco_nozzles(),
        ),        
        World::new(
            mem_card,
            save_file,
            String::from(World::NAME_GELATO),
            WorldIdentifier::GelatoId,
            CoinData::get_gelato_coin_data(),
            BlueCoinData::get_gelato_bcs(),
            ShineData::get_gelato_shines(),
            NozzleData::get_gelato_nozzles(),
        ),
        World::new(
            mem_card,
            save_file,
            String::from(World::NAME_PINNA),
            WorldIdentifier::PinnaId,
            CoinData::get_pinna_coin_data(),
            BlueCoinData::get_pinna_bcs(),
            ShineData::get_pinna_shines(),
            NozzleData::get_pinna_nozzles(),
        ),
        World::new(
            mem_card,
            save_file,
            String::from(World::NAME_SIRENA),
            WorldIdentifier::SirenaId,
            CoinData::get_sirena_coin_data(),
            BlueCoinData::get_sirena_bcs(),
            ShineData::get_sirena_shines(),
            NozzleData::get_sirena_nozzles(),
        ),
        World::new(
            mem_card,
            save_file,
            String::from(World::NAME_NOKI),
            WorldIdentifier::NokiId,
            CoinData::get_noki_coin_data(),
            BlueCoinData::get_noki_bcs(),
            ShineData::get_noki_shines(),
            NozzleData::get_noki_nozzles(),
        ),
        World::new(
            mem_card,
            save_file,
            String::from(World::NAME_PIANTA),
            WorldIdentifier::PiantaId,
            CoinData::get_pianta_coin_data(),
            BlueCoinData::get_pianta_bcs(),
            ShineData::get_pianta_shines(),
            NozzleData::get_pianta_nozzles(),
        ),
        World::new(
            mem_card,
            save_file,
            String::from(World::NAME_CORONA),
            WorldIdentifier::CoronaId,
            CoinData::get_corona_coin_data(),
            BlueCoinData::get_corona_bcs(),
            ShineData::get_corona_shines(),
            NozzleData::get_corona_nozzles(),
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

/// Gets the SVG asset corresponding to blue coin collected state
/// 
/// Returns a path string to the SVG
pub fn get_svg_path_from_blue_coin_collected(is_collected: bool) -> Handle {
    return if is_collected {
        blue_coin_collected_svg_path()
    } else {
        blue_coin_missing_svg_path()
    }
}

/// Gets a list of blue coin lists to be divided into rows
/// 
/// Returns a list of 3 blue coin lists
pub fn get_blue_coin_row_lists(world: &World) -> Vec<Vec<BlueCoin>> {
    let mut count: usize = 0;
    let mut r1: Vec<BlueCoin> = vec![];
    let mut r2: Vec<BlueCoin> = vec![];
    let mut r3: Vec<BlueCoin> = vec![];

    for b in &world.blue_coins {
        // There is a max of 30 blue coins per level, so 10 per row
        match count / 10 {
            0 => r1.push(b.clone()),
            1 => r2.push(b.clone()),
            2 => r3.push(b.clone()),
            _ => { /* Do nothing. Blue Coins should never exceed 30 per level */ }
        };

        count = count +1;
    }

    vec![r1, r2, r3]
}

/// Gets the SVG asset corresponding to blue coin collected state
/// 
/// Returns a path string to the SVG
pub fn get_svg_path_from_shine_collected(is_collected: bool) -> Handle {
    return if is_collected {
        sunshine_collected_svg_path()
    } else {
        sunshine_missing_svg_path()
    }
}

/// Gets a list of shine lists to be divided into rows
/// 
/// Returns a list of 2 shine lists
pub fn get_shine_row_lists(world: &World) -> Vec<Vec<Shine>> {
    let mut count: usize = 0;
    let divider = match world.identifier {
        WorldIdentifier::DelfinoId => 9,
        _ => 8,
    };
    let mut r1: Vec<Shine> = vec![];
    let mut r2: Vec<Shine> = vec![];

    for s in &world.shines {
        match count / divider {
            0 => r1.push(s.clone()),
            1 => r2.push(s.clone()),
            _ => { /* Do nothing */ }
        };

        count = count +1;
    }

    vec![r1, r2]
}

/// Gets the SVG asset corresponding to blue coin collected state
/// 
/// Returns a path string to the SVG
pub fn get_svg_path_from_nozzle_collected(is_collected: bool, nozzle_type: NozzleType) -> Handle {
    if is_collected {
        return match nozzle_type {
            NozzleType::Rocket => rocket_nozzle_collected_svg_path(),
            NozzleType::Turbo => turbo_nozzle_collected_svg_path()
        }
    } else {
        return match nozzle_type {
            NozzleType::Rocket => rocket_nozzle_missing_svg_path(),
            NozzleType::Turbo => turbo_nozzle_missing_svg_path()
        }
    }
}

/// Gets the total blue coin shine count for a save file
/// 
/// Returns the total blue coin shine count
pub fn get_blue_coin_shine_count(save_file: &SaveFile, mem_card: &MemoryCard) -> u16 {
    let mut count: u16 = 0;
    for sd in ShineData::get_blue_coin_shines() {
        count = count + if Shine::from_data(mem_card, save_file, &sd).get_is_collected() {
            1
        } else {
            0
        };
    }

    count
}

/// Reads data from a byte address by comparing 2 save blocks from the memory card
/// 
/// Returns the u16 number from the read memory address
pub fn read_u16_data_from_address(save_address: usize, mem_card: &MemoryCard) -> u16 {
    // Combine 2 bytes into 1 16-bit value using bitwise operations
    // Note: could cause endian problems.
    let val_addr1: u16 = (mem_card.get_mem_data()[save_address - 1] as u16) << 8 | mem_card.get_mem_data()[save_address] as u16;
    let val_addr2: u16 = 
        (mem_card.get_mem_data()[(save_address + 0x2000) - 1] as u16) << 8 | mem_card.get_mem_data()[save_address + 0x2000] as u16;

    if val_addr1 > val_addr2 {
        return val_addr1;
    }
    val_addr2
}