use crate::{data::coin_data::CoinData, models::{mem_card::MemoryCard, save_file::SaveFile}};

use super::blue_coin::BlueCoin;

pub struct World {
    name: String,
    coin_hs: u16,
    blue_coins: Vec<BlueCoin>,
}

impl World {
    fn get_total_coins(mem_card: &MemoryCard, save_file: &SaveFile, address: usize) -> u16 {
        // SMS Saves the total coin high score as 2 bytes.
        // The first byte is used only if the high score exceeds 255 (or 0xFF)
        // The second byte is usually the entire high score since, on most levels, there are less than 255 coins.

        let save_address: usize = address + (save_file.get_coin_hs_byte_offset() as usize);

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

    pub fn get_coin_hs(&self) -> u16 {
        self.coin_hs
    }

    pub fn new(
        mem_card: &MemoryCard,
        save_file: &SaveFile,
        name: String,
        coin_data: CoinData,
        blue_coins: Vec<BlueCoin>
    ) -> World {
        let coin_hs: u16 = World::get_total_coins(mem_card, save_file, coin_data.get_byte() as usize);
        World { name, coin_hs, blue_coins }
    }
}
