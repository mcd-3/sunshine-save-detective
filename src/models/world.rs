use super::blue_coin::BlueCoin;

pub struct World {
    pub name: String,
    coin_hs: u32,
    pub blue_coins: Vec<BlueCoin>,
}

impl World {
    pub fn new(name: String, coin_hs: u32, blue_coins: Vec<BlueCoin>) -> World {
        World { name, coin_hs, blue_coins }
    }
}
