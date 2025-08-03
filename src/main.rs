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

use crate::{data::{blue_coin_data::BlueCoinData, coin_data::CoinData}};

fn main() {
    let mem_card: MemoryCard = MemoryCard::read_from_file("./sms_120.gci");

    // For each world, make a World object. Do this 3 times for each save file

    // save_file_index_offset = (save_file.index * 400)
    // byte_addr = (200 + save_file_index_offset) | (200 + 200 + save_file_index_offset)
    //              0x2000, 0x6000, 0x10000          0x4000, 0x8000, 0x12000
    // To confirm save files B and C

    let save_a: SaveFile = SaveFile::new(
        SaveFile::SAVE_A_LABEL,
        SaveFile::SAVE_A_FILE_INDEX,
        SaveFile::SAVE_A_BC_START,
        SaveFile::SAVE_A_BC_END,
    );

    let save_b: SaveFile = SaveFile::new(
        SaveFile::SAVE_B_LABEL,
        SaveFile::SAVE_B_FILE_INDEX,
        SaveFile::SAVE_B_BC_START,
        SaveFile::SAVE_B_BC_END,
    );

    let save_c: SaveFile = SaveFile::new(
        SaveFile::SAVE_C_LABEL,
        SaveFile::SAVE_C_FILE_INDEX,
        SaveFile::SAVE_C_BC_START,
        SaveFile::SAVE_C_BC_END,
    );

    let w_delfino = World::new(
        &mem_card,
        &save_a,
        String::from("Delfino"),
        CoinData::get_delfino_coin_data(),
        BlueCoinData::get_delfino_bcs()
    );

    let w_delfino_b = World::new(
        &mem_card,
        &save_b,
        String::from("Delfino"),
        CoinData::get_delfino_coin_data(),
        BlueCoinData::get_delfino_bcs()
    );

    let w_delfino_c = World::new(
        &mem_card,
        &save_c,
        String::from("Delfino"),
        CoinData::get_delfino_coin_data(),
        BlueCoinData::get_delfino_bcs()
    );

    println!("Delfino Coin Score: {}", w_delfino.get_coin_hs());
    println!("Delfino Coin Score B: {}", w_delfino_b.get_coin_hs());
    println!("Delfino Coin Score C: {}", w_delfino_c.get_coin_hs());

    println!("Delfino Blue Coins:");
    for i in w_delfino.blue_coins {
        println!("{} | {}", i.get_is_collected(), i.get_description())
    }
}
