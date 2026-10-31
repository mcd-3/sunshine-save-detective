use crate::enums::nozzle_type::NozzleType;

pub struct NozzleData {
    byte: u32,
    bit_index: u32,
    nozzle_type: NozzleType,
}

impl NozzleData {
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

    pub fn new(byte: u32, bit_index: u32, nozzle_type: NozzleType) -> NozzleData {
        NozzleData { byte, bit_index, nozzle_type }
    }

    pub fn get_byte(&self) -> u32 {
        self.byte
    }

    pub fn get_byte_index(&self) -> u32 {
        self.bit_index
    }

    pub fn get_nozzle_type(&self) -> NozzleType {
        self.nozzle_type
    }

    pub fn get_delfino_nozzles() -> Vec<NozzleData> {
        vec![
            NozzleData::new(0x010C, Self::BINARY_INDEX_64, NozzleType::Rocket),
            NozzleData::new(0x010C, Self::BINARY_INDEX_128, NozzleType::Turbo),
        ]
    }

    pub fn get_bianco_nozzles() -> Vec<NozzleData> {
        vec![
            NozzleData::new(0x010D, Self::BINARY_INDEX_1, NozzleType::Rocket),
            NozzleData::new(0x010D, Self::BINARY_INDEX_2, NozzleType::Turbo),
        ]
    }

    pub fn get_ricco_nozzles() -> Vec<NozzleData> {
        vec![
            NozzleData::new(0x010D, Self::BINARY_INDEX_4, NozzleType::Rocket),
            NozzleData::new(0x010D, Self::BINARY_INDEX_8, NozzleType::Turbo),
        ]
    }

    pub fn get_gelato_nozzles() -> Vec<NozzleData> {
        vec![
            NozzleData::new(0x010D, Self::BINARY_INDEX_16, NozzleType::Rocket),
            NozzleData::new(0x010D, Self::BINARY_INDEX_32, NozzleType::Turbo),
        ]
    }

    // Pinna does not have any nozzles
    pub fn get_pinna_nozzles() -> Vec<NozzleData> { vec![] }

    // Sirena does not have any nozzles
    pub fn get_sirena_nozzles() -> Vec<NozzleData> { vec![] }

    pub fn get_noki_nozzles() -> Vec<NozzleData> {
        vec![
            NozzleData::new(0x010E, Self::BINARY_INDEX_16, NozzleType::Rocket),
            NozzleData::new(0x010E, Self::BINARY_INDEX_32, NozzleType::Turbo),
        ]
    }

    pub fn get_pianta_nozzles() -> Vec<NozzleData> {
        vec![
            NozzleData::new(0x010E, Self::BINARY_INDEX_4, NozzleType::Rocket)
        ]
    }

    pub fn get_corona_nozzles() -> Vec<NozzleData> {
        vec![
            NozzleData::new(0x010E, Self::BINARY_INDEX_64, NozzleType::Rocket)
        ]
    }
}