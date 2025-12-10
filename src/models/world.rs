use crate::{
    data::{
        blue_coin_data::BlueCoinData, coin_data::CoinData, nozzle_data::NozzleData, shine_data::ShineData
    },
    enums::world_identifier::WorldIdentifier,
    models::{
        mem_card::MemoryCard, nozzle::Nozzle, save_file::SaveFile, shine::Shine
    },
    utils::data::read_u16_data_from_address
};

use super::blue_coin::BlueCoin;

pub struct World {
    _name: String,
    coin_hs: u16,
    pub blue_coins: Vec<BlueCoin>,
    pub shines: Vec<Shine>,
    pub nozzles: Vec<Nozzle>,
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
        read_u16_data_from_address(save_address, mem_card)
    }

    pub fn get_coin_hs(&self) -> u16 {
        self.coin_hs
    }

    pub fn get_identifier_from_name(str: &str) -> WorldIdentifier {
        match str {
            World::NAME_DELFINO => WorldIdentifier::DelfinoId,
            World::NAME_BIANCO => WorldIdentifier::BiancoId,
            World::NAME_RICCO => WorldIdentifier::RiccoId,
            World::NAME_GELATO => WorldIdentifier::GelatoId,
            World::NAME_PINNA => WorldIdentifier::PinnaId,
            World::NAME_SIRENA => WorldIdentifier::SirenaId,
            World::NAME_NOKI => WorldIdentifier::NokiId,
            World::NAME_PIANTA => WorldIdentifier::PiantaId,
            World::NAME_CORONA => WorldIdentifier::CoronaId,
            _ => WorldIdentifier::DelfinoId
        }
    }

    pub fn get_name_from_identifier(id: WorldIdentifier) -> &'static str {
         match id {
            WorldIdentifier::DelfinoId => World::NAME_DELFINO,
            WorldIdentifier::BiancoId => World::NAME_BIANCO,
            WorldIdentifier::RiccoId => World::NAME_RICCO,
            WorldIdentifier::GelatoId => World::NAME_GELATO,
            WorldIdentifier::PinnaId => World::NAME_PINNA,
            WorldIdentifier::SirenaId => World::NAME_SIRENA,
            WorldIdentifier::NokiId => World::NAME_NOKI,
            WorldIdentifier::PiantaId => World::NAME_PIANTA,
            WorldIdentifier::CoronaId => World::NAME_CORONA,
        }
    }

    pub fn new(
        mem_card: &MemoryCard,
        save_file: &SaveFile,
        name: String,
        world_identifier: WorldIdentifier,
        coin_data: CoinData,
        blue_coin_data: Vec<BlueCoinData>,
        shine_data: Vec<ShineData>,
        nozzle_data: Vec<NozzleData>,
    ) -> World {
        let coin_hs: u16 = World::get_total_coins(mem_card, save_file, coin_data.get_byte() as usize);

        let mut blue_coins: Vec<BlueCoin> = vec![];
        for bcd in blue_coin_data {
            blue_coins.push(BlueCoin::from_data(mem_card, save_file, &bcd));
        }

        let mut shines: Vec<Shine> = vec![];
        for sd in shine_data {
            shines.push(Shine::from_data(mem_card, save_file, &sd));
        }

        let mut nozzles: Vec<Nozzle> = vec![];
        for nd in nozzle_data {
            nozzles.push(Nozzle::from_data(mem_card, save_file, &nd));
        }

        World {
            _name: name,
            coin_hs,
            blue_coins,
            shines,
            nozzles,
            identifier: world_identifier
        }
    }
}
