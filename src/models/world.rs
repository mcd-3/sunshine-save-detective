use crate::{
    data::{
        blue_coin_data::BlueCoinData,
        coin_data::CoinData
    },
    enums::world_identifier::WorldIdentifier,
    models::{
        mem_card::MemoryCard,
        save_file::SaveFile
    }
};

use super::blue_coin::BlueCoin;

pub struct World {
    name: String,
    coin_hs: u16,
    pub blue_coins: Vec<BlueCoin>,
    pub identifier: WorldIdentifier,
}

impl World {
    pub const NAME_DELFINO: &str = "Delfino Plaza";
    pub const NAME_BIANCO: &str = "Bianco Hills";
    pub const NAME_RICCO: &str = "Ricco Harbor";
    pub const NAME_GELATO: &str = "Gelato Beach";
    pub const NAME_PINNA: &str = "Pinna Park";
    pub const NAME_SIRENA: &str = "Sirena Beach";
    pub const NAME_NOKI: &str = "Noki Bay";
    pub const NAME_PIANTA: &str = "Pianta Village";
    pub const NAME_CORONA: &str = "Corona Mountain";

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
        world_identifier: WorldIdentifier,
        coin_data: CoinData,
        blue_coin_data: Vec<BlueCoinData>
    ) -> World {
        let coin_hs: u16 = World::get_total_coins(mem_card, save_file, coin_data.get_byte() as usize);

        let mut blue_coins: Vec<BlueCoin> = vec![];
        for bcd in blue_coin_data {
            blue_coins.push(BlueCoin::from_data(mem_card, save_file, &bcd));
        }

        World { name, coin_hs, blue_coins, identifier: world_identifier }
    }
}
