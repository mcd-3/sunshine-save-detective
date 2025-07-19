pub struct BlueCoinData {
    byte: u32,
    bit_index: u32,
    mpl_order: u8,
    description: String,
}

impl BlueCoinData {
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

    pub fn new(byte: u32, bit_index: u32, mpl_order: u8, description: String) -> BlueCoinData {
        BlueCoinData { byte, bit_index, description, mpl_order }
    }

    pub fn get_byte(&self) -> u32 {
        self.byte
    }

    pub fn get_byte_index(&self) -> u32 {
        self.bit_index
    }

    pub fn get_mpl_order(&self) -> u8 {
        self.mpl_order
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_delfino_bcs() -> Vec<BlueCoinData> {
        vec![
            BlueCoinData::new(0x00AF, Self::BINARY_INDEX_128, 1, String::from("'M' hidden by crate near the pinna park cannon")),
            BlueCoinData::new(0x00AF, Self::BINARY_INDEX_64, 2, String::from("'M' on the tower near the pinna park cannon")),
            BlueCoinData::new(0x00AF, Self::BINARY_INDEX_32, 10, String::from("'M' on the shine gate ")),
            BlueCoinData::new(0x00AF, Self::BINARY_INDEX_16, 16, String::from("Pianta on fire")),
            BlueCoinData::new(0x00AF, Self::BINARY_INDEX_8, 3, String::from("Spray the 'X' near the water")),
            BlueCoinData::new(0x00AF, Self::BINARY_INDEX_4, 4, String::from("Spray the 'X' near Bianco Hills' entrance")),
            BlueCoinData::new(0x00AF, Self::BINARY_INDEX_2, 17, String::from("Behind gate near the rocks")),
            BlueCoinData::new(0x00AF, Self::BINARY_INDEX_1, 20, String::from("Ice cube in Delfino Airstrip")),
            BlueCoinData::new(0x00B0, Self::BINARY_INDEX_128, 18, String::from("Behind yellow gunk near Gelato Beach's entrance")),
            BlueCoinData::new(0x00B0, Self::BINARY_INDEX_64, 11, String::from("In the jail")),
            BlueCoinData::new(0x00B0, Self::BINARY_INDEX_32, 19, String::from("Behind yellow gunk near Pinna Park's entrance")),
            BlueCoinData::new(0x00B0, Self::BINARY_INDEX_16, 7, String::from("Underground near the island")),
            BlueCoinData::new(0x00B0, Self::BINARY_INDEX_8, 15, String::from("Coconut fruit basket")),
            BlueCoinData::new(0x00B0, Self::BINARY_INDEX_4, 12, String::from("Banana fruit basket")),
            BlueCoinData::new(0x00B0, Self::BINARY_INDEX_2, 14, String::from("Durian fruit basket")),
            BlueCoinData::new(0x00B0, Self::BINARY_INDEX_1, 13, String::from("Pineapple fruit basket")),
            BlueCoinData::new(0x00B1, Self::BINARY_INDEX_1, 6, String::from("Blue bird near the crate smash building")),
            BlueCoinData::new(0x00B1, Self::BINARY_INDEX_2, 5, String::from("Blue bird near Serena Beach's entrance")),
            BlueCoinData::new(0x00B1, Self::BINARY_INDEX_4, 8, String::from("Underground near the crate smash building")),
            BlueCoinData::new(0x00B1, Self::BINARY_INDEX_8, 9, String::from("Underground near Corona Mountain")),
        ]
    }
}