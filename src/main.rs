mod models {
    pub mod blue_coin;
    pub mod mem_card;
    pub mod world;
    pub mod save_file;
}

mod data {
    pub mod data;
}

use data::data::get_delfino_bcs;
use models::{mem_card::MemoryCard, save_file::SaveFile, world::World};

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

fn get_blue_coin_collected(save: &MemoryCard, address: usize, index: u32) -> bool {
    // Blue coin flags are stored in 2 addresses:
    // 0x2nnn and 0x4nnn (for File A)
    // The game switches between the two every save and will NOT save at both at the same time
    // The game will always save at the 0x2nnn address first
    // Ex: Get H.S, save -> Addr 1. Save again -> Addr 2. Save a 3rd time -> Addr 1. etc...
    // We need to check which address has the flag enabled (if it is at all)
    let flag = save.sms_save_memory[address] | save.sms_save_memory[address + 0x2000];
    (flag & 2_u8.pow(index)) != 0
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

    let delfino_bc: [[u16; 2]; 20] = [
        [0x20AF, 1],
        [0x20AF, 0],
        [0x20AF, 4],
        [0x20AF, 5],
        [0x20B1, 5],
        [0x20B1, 4],
        [0x20B0, 3],
        [0x20B1, 6],
        [0x20B1, 7],
        [0x20AF, 2],
        [0x20B0, 1],
        [0x20B0, 5],
        [0x20B0, 7],
        [0x20B0, 6],
        [0x20B0, 4],
        [0x20AF, 3],
        [0x20AF, 6],
        [0x20B0, 0],
        [0x20B0, 2],
        [0x20AF, 7],
    ];

    let bianco_bc: [[u16; 2]; 30] = [
        [0x20B7, 0],
        [0x20B5, 3],
        [0x20B5, 2],
        [0x20B8, 1],
        [0x20B7, 3],
        [0x20B7, 5],
        [0x20B7, 4],
        [0x20B8, 2],
        [0x20B7, 1],
        [0x20B7, 2],
        [0x20B5, 5],
        [0x20B5, 4],
        [0x20B6, 3],
        [0x20B5, 0],
        [0x20B8, 0],
        [0x20B7, 7],
        [0x20B5, 1],
        [0x20B8, 7],
        [0x20B6, 0],
        [0x20B6, 5],
        [0x20B6, 6],
        [0x20B7, 6],
        [0x20B6, 4],
        [0x20B6, 1],
        [0x20B8, 6],
        [0x20B8, 5],
        [0x20B8, 4],
        [0x20B6, 2],
        [0x20B8, 3],
        [0x20B6, 7]
    ];

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
        World::get_worlds_with_data()
    );
    // let save_b: SaveFile = SaveFile::new('B', 1, 0x0, 0x0, World::get_defaults());
    // let save_c: SaveFile = SaveFile::new('C', 2, 0x0, 0x0, World::get_defaults());

    for i in save_a.worlds {
        for j in i.blue_coins {
            println!("{}", j.description);
        }
    }
    
    // let bc = get_blue_coin_collected(&save_file, 0x20AF, 3);
    // println!("{}", bc)
}
