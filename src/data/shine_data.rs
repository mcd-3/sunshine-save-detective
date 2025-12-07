pub struct ShineData {
    byte: u32,
    bit_index: u32,
    description: String,
}

impl ShineData {
    // Indexes of binary values
    // Ex: 00000001 = 1 spot = 0, 00100000 = 32 spot = 5, etc...
    const BINARY_INDEX_1: u32 = 0;
    const BINARY_INDEX_2: u32 = 1;
    const BINARY_INDEX_4: u32 = 2;
    const BINARY_INDEX_8: u32 = 3;
    const BINARY_INDEX_16: u32 = 4;
    const BINARY_INDEX_32: u32 = 5;
    const BINARY_INDEX_64: u32 = 6;
    const BINARY_INDEX_128: u32 = 7;

    const SHINE_1_STR: &str = "Shine 1";
    const SHINE_2_STR: &str = "Shine 2";
    const SHINE_3_STR: &str = "Shine 3";
    const SHINE_4_STR: &str = "Shine 4";
    const SHINE_5_STR: &str = "Shine 5";
    const SHINE_6_STR: &str = "Shine 6";
    const SHINE_7_STR: &str = "Shine 7";
    const SHINE_8_STR: &str = "Shine 8";
    const SHINE_SECRET_1_STR: &str = "Secret Shine 1";
    const SHINE_SECRET_2_STR: &str = "Secret Shine 2";
    const SHINE_100_COINS_STR: &str = "100 Coins Shine";

    pub fn new(byte: u32, bit_index: u32, description: String) -> ShineData {
        ShineData { byte, bit_index, description }
    }

        pub fn get_byte(&self) -> u32 {
        self.byte
    }

    pub fn get_byte_index(&self) -> u32 {
        self.bit_index
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_delfino_shines() -> Vec<ShineData> {
        // TODO: Add shines
        vec![]
    }

    pub fn get_bianco_shines() -> Vec<ShineData> {
        vec![
            ShineData::new(0x00A0, Self::BINARY_INDEX_1, String::from(Self::SHINE_1_STR)),
            ShineData::new(0x00A0, Self::BINARY_INDEX_2, String::from(Self::SHINE_2_STR)),
            ShineData::new(0x00A0, Self::BINARY_INDEX_4, String::from(Self::SHINE_3_STR)),
            ShineData::new(0x00A0, Self::BINARY_INDEX_8, String::from(Self::SHINE_4_STR)),
            ShineData::new(0x00A0, Self::BINARY_INDEX_16, String::from(Self::SHINE_5_STR)),
            ShineData::new(0x00A0, Self::BINARY_INDEX_32, String::from(Self::SHINE_6_STR)),
            ShineData::new(0x00A0, Self::BINARY_INDEX_64, String::from(Self::SHINE_7_STR)),
            ShineData::new(0x00A0, Self::BINARY_INDEX_128, String::from(Self::SHINE_8_STR)),

            ShineData::new(0x00A1, Self::BINARY_INDEX_1, String::from(Self::SHINE_SECRET_1_STR)),
            ShineData::new(0x00A1, Self::BINARY_INDEX_2, String::from(Self::SHINE_SECRET_2_STR)),
            ShineData::new(0x00AC, Self::BINARY_INDEX_16, String::from(Self::SHINE_100_COINS_STR)),
        ]
    }

    pub fn get_ricco_shines() -> Vec<ShineData> {
        vec![
            ShineData::new(0x00A1, Self::BINARY_INDEX_4, String::from(Self::SHINE_1_STR)),
            ShineData::new(0x00A1, Self::BINARY_INDEX_8, String::from(Self::SHINE_2_STR)),
            ShineData::new(0x00A1, Self::BINARY_INDEX_16, String::from(Self::SHINE_3_STR)),
            ShineData::new(0x00A1, Self::BINARY_INDEX_32, String::from(Self::SHINE_4_STR)),
            ShineData::new(0x00A1, Self::BINARY_INDEX_64, String::from(Self::SHINE_5_STR)),
            ShineData::new(0x00A1, Self::BINARY_INDEX_128, String::from(Self::SHINE_6_STR)),
            ShineData::new(0x00A2, Self::BINARY_INDEX_1, String::from(Self::SHINE_7_STR)),
            ShineData::new(0x00A2, Self::BINARY_INDEX_2, String::from(Self::SHINE_8_STR)),

            ShineData::new(0x00A2, Self::BINARY_INDEX_4, String::from(Self::SHINE_SECRET_1_STR)),
            ShineData::new(0x00A2, Self::BINARY_INDEX_8, String::from(Self::SHINE_SECRET_2_STR)),
            ShineData::new(0x00AC, Self::BINARY_INDEX_2, String::from(Self::SHINE_100_COINS_STR)),
        ]
    }

    pub fn get_gelato_shines() -> Vec<ShineData> {
        vec![
            ShineData::new(0x00A2, Self::BINARY_INDEX_16, String::from(Self::SHINE_1_STR)),
            ShineData::new(0x00A2, Self::BINARY_INDEX_32, String::from(Self::SHINE_2_STR)),
            ShineData::new(0x00A2, Self::BINARY_INDEX_64, String::from(Self::SHINE_3_STR)),
            ShineData::new(0x00A2, Self::BINARY_INDEX_128, String::from(Self::SHINE_4_STR)),
            ShineData::new(0x00A3, Self::BINARY_INDEX_1, String::from(Self::SHINE_5_STR)),
            ShineData::new(0x00A3, Self::BINARY_INDEX_2, String::from(Self::SHINE_6_STR)),
            ShineData::new(0x00A3, Self::BINARY_INDEX_4, String::from(Self::SHINE_7_STR)),
            ShineData::new(0x00A3, Self::BINARY_INDEX_8, String::from(Self::SHINE_8_STR)),

            ShineData::new(0x00A3, Self::BINARY_INDEX_16, String::from(Self::SHINE_SECRET_1_STR)),
            ShineData::new(0x00A3, Self::BINARY_INDEX_32, String::from(Self::SHINE_SECRET_2_STR)),
            ShineData::new(0x00AC, Self::BINARY_INDEX_64, String::from(Self::SHINE_100_COINS_STR)),
        ]
    }

    pub fn get_pinna_shines() -> Vec<ShineData> {
        vec![
            ShineData::new(0x00A3, Self::BINARY_INDEX_64, String::from(Self::SHINE_1_STR)),
            ShineData::new(0x00A3, Self::BINARY_INDEX_128, String::from(Self::SHINE_2_STR)),
            ShineData::new(0x00A4, Self::BINARY_INDEX_1, String::from(Self::SHINE_3_STR)),
            ShineData::new(0x00A4, Self::BINARY_INDEX_2, String::from(Self::SHINE_4_STR)),
            ShineData::new(0x00A4, Self::BINARY_INDEX_4, String::from(Self::SHINE_5_STR)),
            ShineData::new(0x00A4, Self::BINARY_INDEX_8, String::from(Self::SHINE_6_STR)),
            ShineData::new(0x00A4, Self::BINARY_INDEX_16, String::from(Self::SHINE_7_STR)),
            ShineData::new(0x00A4, Self::BINARY_INDEX_32, String::from(Self::SHINE_8_STR)),

            ShineData::new(0x00A4, Self::BINARY_INDEX_64, String::from(Self::SHINE_SECRET_1_STR)),
            ShineData::new(0x00A4, Self::BINARY_INDEX_128, String::from(Self::SHINE_SECRET_2_STR)),
            ShineData::new(0x00AC, Self::BINARY_INDEX_128, String::from(Self::SHINE_100_COINS_STR)),
        ]
    }

    pub fn get_sirena_shines() -> Vec<ShineData> {
        vec![
            ShineData::new(0x00A5, Self::BINARY_INDEX_1, String::from(Self::SHINE_1_STR)),
            ShineData::new(0x00A5, Self::BINARY_INDEX_2, String::from(Self::SHINE_2_STR)),
            ShineData::new(0x00A5, Self::BINARY_INDEX_4, String::from(Self::SHINE_3_STR)),
            ShineData::new(0x00A5, Self::BINARY_INDEX_8, String::from(Self::SHINE_4_STR)),
            ShineData::new(0x00A5, Self::BINARY_INDEX_16, String::from(Self::SHINE_5_STR)),
            ShineData::new(0x00A5, Self::BINARY_INDEX_32, String::from(Self::SHINE_6_STR)),
            ShineData::new(0x00A5, Self::BINARY_INDEX_64, String::from(Self::SHINE_7_STR)),
            ShineData::new(0x00A5, Self::BINARY_INDEX_128, String::from(Self::SHINE_8_STR)),

            ShineData::new(0x00A6, Self::BINARY_INDEX_1, String::from(Self::SHINE_SECRET_1_STR)),
            ShineData::new(0x00A6, Self::BINARY_INDEX_2, String::from(Self::SHINE_SECRET_2_STR)),
            ShineData::new(0x00AD, Self::BINARY_INDEX_128, String::from(Self::SHINE_100_COINS_STR)),
        ]
    }

    pub fn get_noki_shines() -> Vec<ShineData> {
        vec![
            ShineData::new(0x00A6, Self::BINARY_INDEX_4, String::from(Self::SHINE_1_STR)),
            ShineData::new(0x00A6, Self::BINARY_INDEX_8, String::from(Self::SHINE_2_STR)),
            ShineData::new(0x00A6, Self::BINARY_INDEX_16, String::from(Self::SHINE_3_STR)),
            ShineData::new(0x00A6, Self::BINARY_INDEX_32, String::from(Self::SHINE_4_STR)),
            ShineData::new(0x00A6, Self::BINARY_INDEX_64, String::from(Self::SHINE_5_STR)),
            ShineData::new(0x00A6, Self::BINARY_INDEX_128, String::from(Self::SHINE_6_STR)),
            ShineData::new(0x00A7, Self::BINARY_INDEX_1, String::from(Self::SHINE_7_STR)),
            ShineData::new(0x00A7, Self::BINARY_INDEX_2, String::from(Self::SHINE_8_STR)),

            ShineData::new(0x00A7, Self::BINARY_INDEX_4, String::from(Self::SHINE_SECRET_1_STR)),
            ShineData::new(0x00A7, Self::BINARY_INDEX_8, String::from(Self::SHINE_SECRET_2_STR)),
            ShineData::new(0x00AD, Self::BINARY_INDEX_64, String::from(Self::SHINE_100_COINS_STR)),
        ]
    }

    // TODO: These might be completely wrong! Reverify before adding them!
    pub fn get_pianta_shines() -> Vec<ShineData> {
        vec![
            ShineData::new(0x00B5, Self::BINARY_INDEX_128, String::from(Self::SHINE_1_STR)),
            ShineData::new(0x00B5, Self::BINARY_INDEX_128, String::from(Self::SHINE_2_STR)),
            ShineData::new(0x00B5, Self::BINARY_INDEX_128, String::from(Self::SHINE_3_STR)),
            ShineData::new(0x00B5, Self::BINARY_INDEX_128, String::from(Self::SHINE_4_STR)),
            ShineData::new(0x00B5, Self::BINARY_INDEX_128, String::from(Self::SHINE_5_STR)),
            ShineData::new(0x00B5, Self::BINARY_INDEX_128, String::from(Self::SHINE_6_STR)),
            ShineData::new(0x00B5, Self::BINARY_INDEX_128, String::from(Self::SHINE_7_STR)),
            ShineData::new(0x00B5, Self::BINARY_INDEX_128, String::from(Self::SHINE_8_STR)),

            ShineData::new(0x00B5, Self::BINARY_INDEX_128, String::from(Self::SHINE_SECRET_1_STR)),
            ShineData::new(0x00B5, Self::BINARY_INDEX_128, String::from(Self::SHINE_SECRET_2_STR)),
            ShineData::new(0x00B5, Self::BINARY_INDEX_128, String::from(Self::SHINE_100_COINS_STR)),
        ]
    }

    pub fn get_corona_shines() -> Vec<ShineData> {
        vec![
            ShineData::new(0x00AE, Self::BINARY_INDEX_128, String::from("Final boss")),
        ]
    }
}