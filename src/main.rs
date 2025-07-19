mod models {
    pub mod blue_coin;
    pub mod mem_card;
    pub mod world;
    pub mod save_file;
}

mod data {
    pub mod data;
}

use models::{mem_card::MemoryCard, save_file::SaveFile, world::World};

use crate::{data::data::BlueCoinData, models::blue_coin::BlueCoin};

fn get_total_coins(save: &MemoryCard, address: usize) -> u16 {
    // SMS Saves the total coin high score as 2 bytes.
    // The first byte is used only if the high score exceeds 255 (or 0xFF)
    // The second byte is usually the entire high score since, on most levels, there are less than 255 coins.

    // Combine 2 bytes into 1 16-bit value using bitwise operations
    // Note: could cause endian problems.
    (save.sms_save_memory[address - 1] as u16) << 8 | save.sms_save_memory[address] as u16
}

fn compare(num1: u16, num2: u16) -> u16 {
    if num2 > num1 {
        return num2;
    }
    num1
}

fn main() {
    let save_file: MemoryCard = MemoryCard::read_from_file("./sms_120.gci");

    // Coin high scores are stored in 2 addresses:
    // 0x2nnn and 0x4nnn (for File A)
    // The game switches between the two every save and will NOT save at both at the same time
    // The game will always save at the 0x2nnn address first
    // Ex: Get H.S, save -> Addr 1. Save again -> Addr 2. Save a 3rd time -> Addr 1. etc...
    // We need to check which address has the higher count and display that one
    let coin_hs_addrs: [usize; 16] = [
        0x22BB, 0x42BB, // Delfino
        0x22BF, 0x42BF, // Bianco
        0x22C3, 0x42C3, // Ricco
        0x22C7, 0x42C7, // Gelato
        0x22CB, 0x42CB, // Pinna
        0x22CF, 0x42CF, // Serena
        0x22D7, 0x42D7, // Noki
        0x22D3, 0x42D3, // Pianta
    ];

    let delfino_coins = compare(
        get_total_coins(&save_file, coin_hs_addrs[0]),
        get_total_coins(&save_file, coin_hs_addrs[1]),
    );

    let bianco_coins = compare(
        get_total_coins(&save_file, coin_hs_addrs[2]),
        get_total_coins(&save_file, coin_hs_addrs[3]),
    );

    let ricco_coins = compare(
        get_total_coins(&save_file, coin_hs_addrs[4]),
        get_total_coins(&save_file, coin_hs_addrs[5]),
    );

    let gelato_coins = compare(
        get_total_coins(&save_file, coin_hs_addrs[6]),
        get_total_coins(&save_file, coin_hs_addrs[7]),
    );

    let pinna_coins = compare(
        get_total_coins(&save_file, coin_hs_addrs[8]),
        get_total_coins(&save_file, coin_hs_addrs[9]),
    );

    let serena_coins = compare(
        get_total_coins(&save_file, coin_hs_addrs[10]),
        get_total_coins(&save_file, coin_hs_addrs[11]),
    );

    let noki_coins = compare(
        get_total_coins(&save_file, coin_hs_addrs[12]),
        get_total_coins(&save_file, coin_hs_addrs[13]),
    );

    let pianta_coins = compare(
        get_total_coins(&save_file, coin_hs_addrs[14]),
        get_total_coins(&save_file, coin_hs_addrs[15]),
    );

    println!("Delfino: {}", delfino_coins);
    println!("Bianco: {}", bianco_coins);
    println!("Ricco: {}", ricco_coins);
    println!("Gelato: {}", gelato_coins);
    println!("Pinna: {}", pinna_coins);
    println!("Serena: {}", serena_coins);
    println!("Noki: {}", noki_coins);
    println!("Pianta: {}", pianta_coins);

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
        println!("{} | {}", i.is_collected, i.description)
    }
}
