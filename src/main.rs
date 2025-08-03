mod models {
    pub mod blue_coin;
    pub mod mem_card;
    pub mod world;
    pub mod save_file;
}

mod data {
    pub mod blue_coin_data;
    pub mod coin_data;
}

use models::{mem_card::MemoryCard, save_file::SaveFile, world::World};

use crate::{data::{blue_coin_data::BlueCoinData, coin_data::CoinData}, models::blue_coin::BlueCoin};

fn main() {
    let save_file: MemoryCard = MemoryCard::read_from_file("./sms_120.gci");

    // For each world, make a World object. Do this 3 times for each save file

    // save_file_index_offset = (save_file.index * 400)
    // byte_addr = (200 + save_file_index_offset) | (200 + 200 + save_file_index_offset)
    //              0x2000, 0x6000, 0x10000          0x4000, 0x8000, 0x12000
    // To confirm save files B and C

    let delfino_bcs = BlueCoinData::get_delfino_bcs();
    let bianco_bcs = BlueCoinData::get_biano_bcs();
    let ricco_bcs = BlueCoinData::get_ricco_bcs();
    let gelato_bcs = BlueCoinData::get_gelato_bcs();
    let pinna_bcs = BlueCoinData::get_pinna_bcs();
    let sirena_bcs = BlueCoinData::get_sirena_bcs();
    let nokki_bcs = BlueCoinData::get_nokki_bcs();
    let pianta_bcs = BlueCoinData::get_pianta_bcs();
    let corona_bcs = BlueCoinData::get_corona_bcs();
    let mut blue_coins: Vec<BlueCoin> = vec![];

    let save_a: SaveFile = SaveFile::new(
        SaveFile::SAVE_A_LABEL, 
        SaveFile::SAVE_A_FILE_INDEX, 
        SaveFile::SAVE_A_BC_START, 
        SaveFile::SAVE_A_BC_END, 
    );

    let mut total_bcs_count = 0;


    for bcd in delfino_bcs {
        blue_coins.push(BlueCoin::from_data(&save_file, &save_a, &bcd));
        total_bcs_count = total_bcs_count + 1;
    }

    for bcd in bianco_bcs {
        blue_coins.push(BlueCoin::from_data(&save_file, &save_a, &bcd));
        total_bcs_count = total_bcs_count + 1;
    }

    for bcd in ricco_bcs {
        blue_coins.push(BlueCoin::from_data(&save_file, &save_a, &bcd));
        total_bcs_count = total_bcs_count + 1;
    }

    for bcd in gelato_bcs {
        blue_coins.push(BlueCoin::from_data(&save_file, &save_a, &bcd));
        total_bcs_count = total_bcs_count + 1;
    }

    for bcd in pinna_bcs {
        blue_coins.push(BlueCoin::from_data(&save_file, &save_a, &bcd));
        total_bcs_count = total_bcs_count + 1;
    }

    for bcd in sirena_bcs {
        blue_coins.push(BlueCoin::from_data(&save_file, &save_a, &bcd));
        total_bcs_count = total_bcs_count + 1;
    }

    for bcd in nokki_bcs {
        blue_coins.push(BlueCoin::from_data(&save_file, &save_a, &bcd));
        total_bcs_count = total_bcs_count + 1;
    }

    for bcd in pianta_bcs {
        blue_coins.push(BlueCoin::from_data(&save_file, &save_a, &bcd));
        total_bcs_count = total_bcs_count + 1;
    }

    for bcd in corona_bcs {
        blue_coins.push(BlueCoin::from_data(&save_file, &save_a, &bcd));
        total_bcs_count = total_bcs_count + 1;
    }

    println!("Total: {}", total_bcs_count);


    for i in blue_coins {
        println!("{} | {}", i.get_is_collected(), i.get_description())
    }

    let w_delfino = World::new(&save_file, &save_a, String::from("Delfino"), CoinData::get_delfino_coin_data(), vec![]);
    let w_bianco = World::new(&save_file, &save_a, String::from("Bianco"), CoinData::get_bianco_coin_data(), vec![]);
    let w_ricco = World::new(&save_file, &save_a, String::from("Ricco"), CoinData::get_ricco_coin_data(), vec![]);
    println!("Delfino: {}", w_delfino.get_coin_hs());
    println!("Bianco: {}", w_bianco.get_coin_hs());
    println!("Ricco: {}", w_ricco.get_coin_hs());
    println!("Delfino: {}", w_delfino.get_coin_hs());
}
