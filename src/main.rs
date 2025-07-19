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
    let mut blue_coins: Vec<BlueCoin> = vec![];
    // let pianta_bcs = get_delfino_bcs();

    // let delfino_w = World::new(String::from("asd"), 0, delfino_bcs);
    // let pianta_w = World::new(String::from("asd"), 0, pianta_bcs);

    let save_a: SaveFile = SaveFile::new(
        SaveFile::SAVE_A_LABEL, 
        SaveFile::SAVE_A_FILE_INDEX, 
        SaveFile::SAVE_A_BC_START, 
        SaveFile::SAVE_A_BC_END, 
    );


    for bcd in delfino_bcs {
        blue_coins.push(BlueCoin::from_data(&save_file, &save_a, &bcd));
    }

    for bcd in bianco_bcs {
        blue_coins.push(BlueCoin::from_data(&save_file, &save_a, &bcd));
    }

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
