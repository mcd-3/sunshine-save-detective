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

    pub fn get_biano_bcs() -> Vec<BlueCoinData> {
        vec![
            BlueCoinData::new(0x00B5, Self::BINARY_INDEX_128, 14, String::from("Pianta in cavity")),
            BlueCoinData::new(0x00B5, Self::BINARY_INDEX_64, 17, String::from("Palm tree near ropes")),
            BlueCoinData::new(0x00B5, Self::BINARY_INDEX_32, 3, String::from("Spray water in right doorway of the rocket nozzle building in the village")),
            BlueCoinData::new(0x00B5, Self::BINARY_INDEX_16, 2, String::from("Building with 2 towers")),
            BlueCoinData::new(0x00B5, Self::BINARY_INDEX_8, 12, String::from("Spray stone pillar near large windmill")),
            BlueCoinData::new(0x00B5, Self::BINARY_INDEX_4, 11, String::from("'M' on big windmill path")),
            BlueCoinData::new(0x00B6, Self::BINARY_INDEX_128, 19, String::from("Under chain bridge")),
            BlueCoinData::new(0x00B6, Self::BINARY_INDEX_64, 24, String::from("Blue bird in village")),
            BlueCoinData::new(0x00B6, Self::BINARY_INDEX_32, 28, String::from("Underwater in windmill lake")),
            BlueCoinData::new(0x00B6, Self::BINARY_INDEX_16, 13, String::from("Underwater in windmill lake")),
            BlueCoinData::new(0x00B6, Self::BINARY_INDEX_8, 23, String::from("Past the cloud platforms")),
            BlueCoinData::new(0x00B6, Self::BINARY_INDEX_4, 20, String::from("Hillside brick platform Pokey")),
            BlueCoinData::new(0x00B6, Self::BINARY_INDEX_2, 21, String::from("Border wall pianta")),
            BlueCoinData::new(0x00B6, Self::BINARY_INDEX_1, 30, String::from("Beehive near chain bridge")),
            BlueCoinData::new(0x00B7, Self::BINARY_INDEX_128, 1, String::from("Underwater near the dam")),
            BlueCoinData::new(0x00B7, Self::BINARY_INDEX_64, 9, String::from("'X' near lakeside wall")),
            BlueCoinData::new(0x00B7, Self::BINARY_INDEX_32, 10, String::from("Spray wind wheel")),
            BlueCoinData::new(0x00B7, Self::BINARY_INDEX_16, 5, String::from("'M' behind 2 tower house")),
            BlueCoinData::new(0x00B7, Self::BINARY_INDEX_8, 7, String::from("'M' on top of village border wall")),
            BlueCoinData::new(0x00B7, Self::BINARY_INDEX_4, 6, String::from("'M' behind waterwheel wall")),
            BlueCoinData::new(0x00B7, Self::BINARY_INDEX_2, 22, String::from("Petey Pianta wooden platform")),
            BlueCoinData::new(0x00B7, Self::BINARY_INDEX_1, 16, String::from("On top of large windmill")),
            BlueCoinData::new(0x00B8, Self::BINARY_INDEX_128, 15, String::from("Tallest wooden pillar")),
            BlueCoinData::new(0x00B8, Self::BINARY_INDEX_64, 4, String::from("Spinning bridge")),
            BlueCoinData::new(0x00B8, Self::BINARY_INDEX_32, 8, String::from("'X' in wall cavity")),
            BlueCoinData::new(0x00B8, Self::BINARY_INDEX_16, 29, String::from("Blue butterflies")),
            BlueCoinData::new(0x00B8, Self::BINARY_INDEX_8, 27, String::from("Circle paint near river")),
            BlueCoinData::new(0x00B8, Self::BINARY_INDEX_4, 26, String::from("Circle paint near throwing Pianta")),
            BlueCoinData::new(0x00B8, Self::BINARY_INDEX_2, 25, String::from("Throwing Pianta")),
            BlueCoinData::new(0x00B8, Self::BINARY_INDEX_1, 18, String::from("Ledge near village")),
        ]
    }
}