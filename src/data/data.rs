pub struct BlueCoinData {
    byte: u32,
    bit_index: u32,
    mpl_order: u8,
    description: String,
}

impl BlueCoinData {
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
            BlueCoinData::new(0x00AF, 0, 1, String::from("'M' hidden by crate near the pinna park cannon")),
            BlueCoinData::new(0x00AF, 1, 2, String::from("'M' on the tower near the pinna park cannon")),
            BlueCoinData::new(0x00AF, 2, 10, String::from("'M' on the shine gate ")),
            BlueCoinData::new(0x00AF, 3, 16, String::from("Pianta on fire")),
            BlueCoinData::new(0x00AF, 4, 3, String::from("Spray the 'X' near the water")),
            BlueCoinData::new(0x00AF, 5, 4, String::from("Spray the 'X' near Bianco Hills' entrance")),
            BlueCoinData::new(0x00AF, 6, 17, String::from("Behind gate near the rocks")),
            BlueCoinData::new(0x00AF, 7, 20, String::from("Ice cube in Delfino Airstrip")),
            BlueCoinData::new(0x00B0, 0, 18, String::from("Behind yellow gunk near Gelato Beach's entrance")),
            BlueCoinData::new(0x00B0, 1, 11, String::from("In the jail")),
            BlueCoinData::new(0x00B0, 2, 19, String::from("Behind yellow gunk near Pinna Park's entrance")),
            BlueCoinData::new(0x00B0, 3, 7, String::from("Underground near the island")),
            BlueCoinData::new(0x00B0, 4, 15, String::from("Coconut fruit basket")),
            BlueCoinData::new(0x00B0, 5, 12, String::from("Banana fruit basket")),
            BlueCoinData::new(0x00B0, 6, 14, String::from("Durian fruit basket")),
            BlueCoinData::new(0x00B0, 7, 13, String::from("Pineapple fruit basket")),
            BlueCoinData::new(0x00B1, 4, 6, String::from("Blue bird near the crate smash building")),
            BlueCoinData::new(0x00B1, 5, 5, String::from("Blue bird near Serena Beach's entrance")),
            BlueCoinData::new(0x00B1, 6, 8, String::from("Underground near the crate smash building")),
            BlueCoinData::new(0x00B1, 7, 9, String::from("Underground near Corona Mountain")),
        ]
    }
}