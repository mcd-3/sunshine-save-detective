use crate::data::data::get_delfino_bcs;

use super::blue_coin::BlueCoin;

pub struct World {
    pub name: String,
    coin_hs: u32,
    pub blue_coins: Vec<BlueCoin>,
}

impl World {
    pub fn get_worlds_with_data() -> Vec<World> {
        vec![
            World { name: String::from("Delfino Plaza"), coin_hs: 0, blue_coins: get_delfino_bcs() },
            World { name: String::from("Bianco Hills"), coin_hs: 0, blue_coins: vec![] },
            World { name: String::from("Ricco Harbor"), coin_hs: 0, blue_coins: vec![] },
            World { name: String::from("Gelato Beach"), coin_hs: 0, blue_coins: vec![] },
            World { name: String::from("Pinna Park"), coin_hs: 0, blue_coins: vec![] },
            World { name: String::from("Sirena Beach"), coin_hs: 0, blue_coins: vec![] },
            World { name: String::from("Noki Bay"), coin_hs: 0, blue_coins: vec![] },
            World { name: String::from("Pianta Village"), coin_hs: 0, blue_coins: vec![] },
            World { name: String::from("Corona Mountain"), coin_hs: 0, blue_coins: vec![] },
        ]
    }
}
