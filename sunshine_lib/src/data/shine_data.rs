pub struct ShineData {
    byte: u32,
    bit_index: u32,
    description: String,
    is_secret: bool,
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
    const SHINE_BC_STR: &str = "Blue Coin Shine";

    pub fn new(byte: u32, bit_index: u32, description: String, is_secret: bool) -> ShineData {
        ShineData { byte, bit_index, description, is_secret }
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

    pub fn get_is_secret(&self) -> bool {
        self.is_secret
    }

    pub fn get_delfino_shines() -> Vec<ShineData> {
        vec![
            ShineData::new(0x00AA, Self::BINARY_INDEX_64, String::from("Air strip"), false),
            ShineData::new(0x00AA, Self::BINARY_INDEX_128, String::from("Police gate"), false),
            ShineData::new(0x00AB, Self::BINARY_INDEX_1, String::from("Airstrip red coins"), false),
            ShineData::new(0x00AB, Self::BINARY_INDEX_2, String::from("Pachinko"), false),
            ShineData::new(0x00AB, Self::BINARY_INDEX_4, String::from("Stone pillars pipe"), false),
            ShineData::new(0x00AB, Self::BINARY_INDEX_8, String::from("Desert island pipe"), false),
            ShineData::new(0x00AB, Self::BINARY_INDEX_16, String::from("Cliff trees pipe"), false),
            ShineData::new(0x00AB, Self::BINARY_INDEX_32, String::from("Lighthouse groundpound"), false),
            ShineData::new(0x00AB, Self::BINARY_INDEX_64, String::from("Crates 1"), false),
            ShineData::new(0x00AB, Self::BINARY_INDEX_128, String::from("Crates 2"), false),
            ShineData::new(0x00AC, Self::BINARY_INDEX_1, String::from("Tower bell"), false),
            ShineData::new(0x00AC, Self::BINARY_INDEX_2, String::from("Jail bell"), false),
            ShineData::new(0x00AC, Self::BINARY_INDEX_8, String::from("Chuckster"), false),
            ShineData::new(0x00AC, Self::BINARY_INDEX_16, String::from("Shine gate"), false),
            ShineData::new(0x00AE, Self::BINARY_INDEX_16, String::from("Under bell tower"), false),
            ShineData::new(0x00AE, Self::BINARY_INDEX_32, String::from("Beach"), false),
            ShineData::new(0x00AE, Self::BINARY_INDEX_64, String::from("Desert island bird"), false),
            // Putting the 100 coins shine last because it makes UX sense even though it's not
            // the correct order in memory
            ShineData::new(0x00AD, Self::BINARY_INDEX_1, String::from("100 coins"), true),
        ]
    }

    pub fn get_blue_coin_shines() -> Vec<ShineData> {
        vec![
            ShineData::new(0x00A8, Self::BINARY_INDEX_64, format!("{} {}", Self::SHINE_BC_STR, 1), false),
            ShineData::new(0x00A8, Self::BINARY_INDEX_128, format!("{} {}", Self::SHINE_BC_STR, 2), false),
            ShineData::new(0x00A9, Self::BINARY_INDEX_1, format!("{} {}", Self::SHINE_BC_STR, 3), false),
            ShineData::new(0x00A9, Self::BINARY_INDEX_2, format!("{} {}", Self::SHINE_BC_STR, 4), false),
            ShineData::new(0x00A9, Self::BINARY_INDEX_4, format!("{} {}", Self::SHINE_BC_STR, 5), false),
            ShineData::new(0x00A9, Self::BINARY_INDEX_8, format!("{} {}", Self::SHINE_BC_STR, 6), false),
            ShineData::new(0x00A9, Self::BINARY_INDEX_16, format!("{} {}", Self::SHINE_BC_STR, 7), false),
            ShineData::new(0x00A9, Self::BINARY_INDEX_32, format!("{} {}", Self::SHINE_BC_STR, 8), false),
            ShineData::new(0x00A9, Self::BINARY_INDEX_64, format!("{} {}", Self::SHINE_BC_STR, 9), false),
            ShineData::new(0x00A9, Self::BINARY_INDEX_128, format!("{} {}", Self::SHINE_BC_STR, 10), false),
            ShineData::new(0x00AA, Self::BINARY_INDEX_1, format!("{} {}", Self::SHINE_BC_STR, 11), false),
            ShineData::new(0x00AA, Self::BINARY_INDEX_2, format!("{} {}", Self::SHINE_BC_STR, 12), false),
            ShineData::new(0x00AA, Self::BINARY_INDEX_4, format!("{} {}", Self::SHINE_BC_STR, 13), false),
            ShineData::new(0x00AA, Self::BINARY_INDEX_8, format!("{} {}", Self::SHINE_BC_STR, 14), false),
            ShineData::new(0x00AA, Self::BINARY_INDEX_16, format!("{} {}", Self::SHINE_BC_STR, 15), false),
            ShineData::new(0x00AA, Self::BINARY_INDEX_32, format!("{} {}", Self::SHINE_BC_STR, 16), false),
            ShineData::new(0x00AD, Self::BINARY_INDEX_16, format!("{} {}", Self::SHINE_BC_STR, 17), false),
            ShineData::new(0x00AD, Self::BINARY_INDEX_32, format!("{} {}", Self::SHINE_BC_STR, 18), false),
            ShineData::new(0x00AD, Self::BINARY_INDEX_64, format!("{} {}", Self::SHINE_BC_STR, 19), false),
            ShineData::new(0x00AD, Self::BINARY_INDEX_128, format!("{} {}", Self::SHINE_BC_STR, 20), false),
            ShineData::new(0x00AE, Self::BINARY_INDEX_1, format!("{} {}", Self::SHINE_BC_STR, 21), false),
            ShineData::new(0x00AE, Self::BINARY_INDEX_2, format!("{} {}", Self::SHINE_BC_STR, 22), false),
            ShineData::new(0x00AE, Self::BINARY_INDEX_4, format!("{} {}", Self::SHINE_BC_STR, 23), false),
            ShineData::new(0x00AE, Self::BINARY_INDEX_8, format!("{} {}", Self::SHINE_BC_STR, 24), false),
        ]
    }

    pub fn get_bianco_shines() -> Vec<ShineData> {
        vec![
            ShineData::new(0x00A0, Self::BINARY_INDEX_1, String::from(Self::SHINE_1_STR), false),
            ShineData::new(0x00A0, Self::BINARY_INDEX_2, String::from(Self::SHINE_2_STR), false),
            ShineData::new(0x00A0, Self::BINARY_INDEX_4, String::from(Self::SHINE_3_STR), false),
            ShineData::new(0x00A0, Self::BINARY_INDEX_8, String::from(Self::SHINE_4_STR), false),
            ShineData::new(0x00A0, Self::BINARY_INDEX_16, String::from(Self::SHINE_5_STR), false),
            ShineData::new(0x00A0, Self::BINARY_INDEX_32, String::from(Self::SHINE_6_STR), false),
            ShineData::new(0x00A0, Self::BINARY_INDEX_64, String::from(Self::SHINE_7_STR), false),
            ShineData::new(0x00A0, Self::BINARY_INDEX_128, String::from(Self::SHINE_8_STR), false),

            ShineData::new(0x00A1, Self::BINARY_INDEX_1, String::from(Self::SHINE_SECRET_1_STR), true),
            ShineData::new(0x00A1, Self::BINARY_INDEX_2, String::from(Self::SHINE_SECRET_2_STR), true),
            ShineData::new(0x00AC, Self::BINARY_INDEX_16, String::from(Self::SHINE_100_COINS_STR), true),
        ]
    }

    pub fn get_ricco_shines() -> Vec<ShineData> {
        vec![
            ShineData::new(0x00A1, Self::BINARY_INDEX_4, String::from(Self::SHINE_1_STR), false),
            ShineData::new(0x00A1, Self::BINARY_INDEX_8, String::from(Self::SHINE_2_STR), false),
            ShineData::new(0x00A1, Self::BINARY_INDEX_16, String::from(Self::SHINE_3_STR), false),
            ShineData::new(0x00A1, Self::BINARY_INDEX_32, String::from(Self::SHINE_4_STR), false),
            ShineData::new(0x00A1, Self::BINARY_INDEX_64, String::from(Self::SHINE_5_STR), false),
            ShineData::new(0x00A1, Self::BINARY_INDEX_128, String::from(Self::SHINE_6_STR), false),
            ShineData::new(0x00A2, Self::BINARY_INDEX_1, String::from(Self::SHINE_7_STR), false),
            ShineData::new(0x00A2, Self::BINARY_INDEX_2, String::from(Self::SHINE_8_STR), false),

            ShineData::new(0x00A2, Self::BINARY_INDEX_4, String::from(Self::SHINE_SECRET_1_STR), true),
            ShineData::new(0x00A2, Self::BINARY_INDEX_8, String::from(Self::SHINE_SECRET_2_STR), true),
            ShineData::new(0x00AC, Self::BINARY_INDEX_2, String::from(Self::SHINE_100_COINS_STR), true),
        ]
    }

    pub fn get_gelato_shines() -> Vec<ShineData> {
        vec![
            ShineData::new(0x00A2, Self::BINARY_INDEX_16, String::from(Self::SHINE_1_STR), false),
            ShineData::new(0x00A2, Self::BINARY_INDEX_32, String::from(Self::SHINE_2_STR), false),
            ShineData::new(0x00A2, Self::BINARY_INDEX_64, String::from(Self::SHINE_3_STR), false),
            ShineData::new(0x00A2, Self::BINARY_INDEX_128, String::from(Self::SHINE_4_STR), false),
            ShineData::new(0x00A3, Self::BINARY_INDEX_1, String::from(Self::SHINE_5_STR), false),
            ShineData::new(0x00A3, Self::BINARY_INDEX_2, String::from(Self::SHINE_6_STR), false),
            ShineData::new(0x00A3, Self::BINARY_INDEX_4, String::from(Self::SHINE_7_STR), false),
            ShineData::new(0x00A3, Self::BINARY_INDEX_8, String::from(Self::SHINE_8_STR), false),

            ShineData::new(0x00A3, Self::BINARY_INDEX_16, String::from(Self::SHINE_SECRET_1_STR), true),
            ShineData::new(0x00A3, Self::BINARY_INDEX_32, String::from(Self::SHINE_SECRET_2_STR), true),
            ShineData::new(0x00AC, Self::BINARY_INDEX_64, String::from(Self::SHINE_100_COINS_STR), true),
        ]
    }

    pub fn get_pinna_shines() -> Vec<ShineData> {
        vec![
            ShineData::new(0x00A3, Self::BINARY_INDEX_64, String::from(Self::SHINE_1_STR), false),
            ShineData::new(0x00A3, Self::BINARY_INDEX_128, String::from(Self::SHINE_2_STR), false),
            ShineData::new(0x00A4, Self::BINARY_INDEX_1, String::from(Self::SHINE_3_STR), false),
            ShineData::new(0x00A4, Self::BINARY_INDEX_2, String::from(Self::SHINE_4_STR), false),
            ShineData::new(0x00A4, Self::BINARY_INDEX_4, String::from(Self::SHINE_5_STR), false),
            ShineData::new(0x00A4, Self::BINARY_INDEX_8, String::from(Self::SHINE_6_STR), false),
            ShineData::new(0x00A4, Self::BINARY_INDEX_16, String::from(Self::SHINE_7_STR), false),
            ShineData::new(0x00A4, Self::BINARY_INDEX_32, String::from(Self::SHINE_8_STR), false),

            ShineData::new(0x00A4, Self::BINARY_INDEX_64, String::from(Self::SHINE_SECRET_1_STR), true),
            ShineData::new(0x00A4, Self::BINARY_INDEX_128, String::from(Self::SHINE_SECRET_2_STR), true),
            ShineData::new(0x00AC, Self::BINARY_INDEX_128, String::from(Self::SHINE_100_COINS_STR), true),
        ]
    }

    pub fn get_sirena_shines() -> Vec<ShineData> {
        vec![
            ShineData::new(0x00A5, Self::BINARY_INDEX_1, String::from(Self::SHINE_1_STR), false),
            ShineData::new(0x00A5, Self::BINARY_INDEX_2, String::from(Self::SHINE_2_STR), false),
            ShineData::new(0x00A5, Self::BINARY_INDEX_4, String::from(Self::SHINE_3_STR), false),
            ShineData::new(0x00A5, Self::BINARY_INDEX_8, String::from(Self::SHINE_4_STR), false),
            ShineData::new(0x00A5, Self::BINARY_INDEX_16, String::from(Self::SHINE_5_STR), false),
            ShineData::new(0x00A5, Self::BINARY_INDEX_32, String::from(Self::SHINE_6_STR), false),
            ShineData::new(0x00A5, Self::BINARY_INDEX_64, String::from(Self::SHINE_7_STR), false),
            ShineData::new(0x00A5, Self::BINARY_INDEX_128, String::from(Self::SHINE_8_STR), false),

            ShineData::new(0x00A6, Self::BINARY_INDEX_1, String::from(Self::SHINE_SECRET_1_STR), true),
            ShineData::new(0x00A6, Self::BINARY_INDEX_2, String::from(Self::SHINE_SECRET_2_STR), true),
            ShineData::new(0x00AD, Self::BINARY_INDEX_128, String::from(Self::SHINE_100_COINS_STR), true),
        ]
    }

    pub fn get_noki_shines() -> Vec<ShineData> {
        vec![
            ShineData::new(0x00A6, Self::BINARY_INDEX_4, String::from(Self::SHINE_1_STR), false),
            ShineData::new(0x00A6, Self::BINARY_INDEX_8, String::from(Self::SHINE_2_STR), false),
            ShineData::new(0x00A6, Self::BINARY_INDEX_16, String::from(Self::SHINE_3_STR), false),
            ShineData::new(0x00A6, Self::BINARY_INDEX_32, String::from(Self::SHINE_4_STR), false),
            ShineData::new(0x00A6, Self::BINARY_INDEX_64, String::from(Self::SHINE_5_STR), false),
            ShineData::new(0x00A6, Self::BINARY_INDEX_128, String::from(Self::SHINE_6_STR), false),
            ShineData::new(0x00A7, Self::BINARY_INDEX_1, String::from(Self::SHINE_7_STR), false),
            ShineData::new(0x00A7, Self::BINARY_INDEX_2, String::from(Self::SHINE_8_STR), false),

            ShineData::new(0x00A7, Self::BINARY_INDEX_4, String::from(Self::SHINE_SECRET_1_STR), true),
            ShineData::new(0x00A7, Self::BINARY_INDEX_8, String::from(Self::SHINE_SECRET_2_STR), true),
            ShineData::new(0x00AD, Self::BINARY_INDEX_64, String::from(Self::SHINE_100_COINS_STR), true),
        ]
    }

    // Note: This is the only world where the memory order is different
    pub fn get_pianta_shines() -> Vec<ShineData> {
        vec![
            ShineData::new(0x00A7, Self::BINARY_INDEX_16, String::from(Self::SHINE_1_STR), false),
            ShineData::new(0x00A8, Self::BINARY_INDEX_2, String::from(Self::SHINE_2_STR), false),
            ShineData::new(0x00A7, Self::BINARY_INDEX_64, String::from(Self::SHINE_3_STR), false),
            ShineData::new(0x00A7, Self::BINARY_INDEX_32, String::from(Self::SHINE_4_STR), false),
            ShineData::new(0x00A8, Self::BINARY_INDEX_1, String::from(Self::SHINE_5_STR), false),
            ShineData::new(0x00A7, Self::BINARY_INDEX_128, String::from(Self::SHINE_6_STR), false),
            ShineData::new(0x00A8, Self::BINARY_INDEX_4, String::from(Self::SHINE_7_STR), false),
            ShineData::new(0x00A8, Self::BINARY_INDEX_8, String::from(Self::SHINE_8_STR), false),

            ShineData::new(0x00A8, Self::BINARY_INDEX_16, String::from(Self::SHINE_SECRET_1_STR), true),
            ShineData::new(0x00A8, Self::BINARY_INDEX_32, String::from(Self::SHINE_SECRET_2_STR), true),
            ShineData::new(0x00AD, Self::BINARY_INDEX_4, String::from(Self::SHINE_100_COINS_STR), true),
        ]
    }

    pub fn get_corona_shines() -> Vec<ShineData> {
        vec![
            ShineData::new(0x00AE, Self::BINARY_INDEX_128, String::from("Final boss"), false),
        ]
    }
}