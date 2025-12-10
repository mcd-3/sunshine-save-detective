use crate::{
    models::mem_card::MemoryCard,
    utils::data::read_u16_data_from_address
};

pub struct SaveFile {
    _label: char,
    pub index: u8,   
}

impl SaveFile {
    pub const SAVE_A_LABEL: char = 'A';
    pub const SAVE_B_LABEL: char = 'B';
    pub const SAVE_C_LABEL: char = 'C';

    pub const SAVE_A_FILE_INDEX: u8 = 0;
    pub const SAVE_B_FILE_INDEX: u8 = 1;
    pub const SAVE_C_FILE_INDEX: u8 = 2;

    pub fn new(label: char, index: u8 ) -> Self {
        SaveFile { _label: label, index }
    }

    pub fn new_by_label(label: char) -> Self {
        match label {
            SaveFile::SAVE_A_LABEL => {
                SaveFile::new(
                    SaveFile::SAVE_A_LABEL,
                    SaveFile::SAVE_A_FILE_INDEX
                )
            },
            SaveFile::SAVE_B_LABEL => {
                SaveFile::new(
                    SaveFile::SAVE_B_LABEL,
                    SaveFile::SAVE_B_FILE_INDEX
                )
            },
            SaveFile::SAVE_C_LABEL => {
                SaveFile::new(
                    SaveFile::SAVE_C_LABEL,
                    SaveFile::SAVE_C_FILE_INDEX
                )
            },
            _ => {
                SaveFile::new(
                    SaveFile::SAVE_A_LABEL,
                    SaveFile::SAVE_A_FILE_INDEX
                )
            }
        }
    }

    pub fn get_data_start_offeset(&self) -> u32 {
        match self.index {
            SaveFile::SAVE_A_FILE_INDEX => 0x2000,
            SaveFile::SAVE_B_FILE_INDEX => 0x6000,
            SaveFile::SAVE_C_FILE_INDEX => 0xA000,
            _ => 0x2000
        }
    }

    pub fn get_coin_hs_byte_offset(&self) -> u32 {
        match self.index {
            SaveFile::SAVE_A_FILE_INDEX => 0x2200,
            SaveFile::SAVE_B_FILE_INDEX => 0x6200,
            SaveFile::SAVE_C_FILE_INDEX => 0xA200,
            _ => 0x2200,
        }
    }

    fn get_shine_count_byte_offset(&self) -> u32 {
        match self.index {
            SaveFile::SAVE_A_FILE_INDEX => 0x205D,
            SaveFile::SAVE_B_FILE_INDEX => 0x605D,
            SaveFile::SAVE_C_FILE_INDEX => 0xA05D,
            _ => 0x205D,
        }
    }

    fn get_lives_count_byte_offset(&self) -> u32 {
        match self.index {
            SaveFile::SAVE_A_FILE_INDEX => 0x22A7,
            SaveFile::SAVE_B_FILE_INDEX => 0x62A7,
            SaveFile::SAVE_C_FILE_INDEX => 0xA2A7,
            _ => 0x22A7,
        }
    }

    pub fn get_lives_count(&self, mem_card: &MemoryCard) -> u16 {
        let save_address: usize = self.get_lives_count_byte_offset() as usize;
        read_u16_data_from_address(save_address, mem_card)
    }

    pub fn get_shine_count(&self, mem_card: &MemoryCard) -> u16 {
        let save_address: usize = self.get_shine_count_byte_offset() as usize;
        read_u16_data_from_address(save_address, mem_card)
    }
}

impl Default for SaveFile {
    fn default() -> Self {
        Self {
            _label: SaveFile::SAVE_A_LABEL,
            index: SaveFile::SAVE_A_FILE_INDEX,
        }
    }
}