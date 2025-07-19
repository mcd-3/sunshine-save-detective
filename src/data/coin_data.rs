pub struct CoinData {
    byte: u32
}

impl CoinData {
    pub fn new(byte: u32) -> CoinData {
        CoinData { byte }
    }

    pub fn get_byte(&self) -> u32 {
        self.byte
    }

    pub fn get_delfino_coin_data() -> CoinData {
        CoinData::new(0x00BB)
    }

    pub fn get_bianco_coin_data() -> CoinData {
        CoinData::new(0x00BF)
    }

    pub fn get_ricco_coin_data() -> CoinData {
        CoinData::new(0x00C3)
    }

    pub fn get_gelato_coin_data() -> CoinData {
        CoinData::new(0x00C7)
    }

    pub fn get_pinna_coin_data() -> CoinData {
        CoinData::new(0x00CB)
    }

    pub fn get_serena_coin_data() -> CoinData {
        CoinData::new(0x00CF)
    }

    pub fn get_noki_coin_data() -> CoinData {
        CoinData::new(0x00D7)
    }

    pub fn get_pianta_coin_data() -> CoinData {
        CoinData::new(0x00D3)
    }
}