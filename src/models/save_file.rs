pub struct SaveFile {
    label: char,
    pub index: u8,
    bc_start_byte: u32,
    bc_end_byte: u32,   
}

impl SaveFile {
    pub const SAVE_A_BC_START: u32 = 0x20AF;
    pub const SAVE_A_BC_END: u32 = 0x20E4;
    pub const SAVE_B_BC_START: u32 = 0x60AF;
    pub const SAVE_B_BC_END: u32 = 0x60E4;
    pub const SAVE_C_BC_START: u32 = 0xA0AF;
    pub const SAVE_C_BC_END: u32 = 0xA0E4;

    pub const SAVE_A_LABEL: char = 'A';
    pub const SAVE_B_LABEL: char = 'B';
    pub const SAVE_C_LABEL: char = 'C';

    pub const SAVE_A_FILE_INDEX: u8 = 0;
    pub const SAVE_B_FILE_INDEX: u8 = 1;
    pub const SAVE_C_FILE_INDEX: u8 = 2;

    pub fn new(label: char, index: u8, bc_start_byte: u32, bc_end_byte: u32 ) -> Self {
        SaveFile { label, index, bc_start_byte, bc_end_byte }
    }

    pub fn new_by_label(label: char) -> Self {
        match label {
            SaveFile::SAVE_A_LABEL => {
                SaveFile::new(
                    SaveFile::SAVE_A_LABEL,
                    SaveFile::SAVE_A_FILE_INDEX,
                    SaveFile::SAVE_A_BC_START,
                    SaveFile::SAVE_A_BC_END
                )
            },
            SaveFile::SAVE_B_LABEL => {
                SaveFile::new(
                    SaveFile::SAVE_B_LABEL,
                    SaveFile::SAVE_B_FILE_INDEX,
                    SaveFile::SAVE_B_BC_START,
                    SaveFile::SAVE_B_BC_END
                )
            },
            SaveFile::SAVE_C_LABEL => {
                SaveFile::new(
                    SaveFile::SAVE_C_LABEL,
                    SaveFile::SAVE_C_FILE_INDEX,
                    SaveFile::SAVE_C_BC_START,
                    SaveFile::SAVE_C_BC_END
                )
            },
            _ => {
                SaveFile::new(
                    SaveFile::SAVE_A_LABEL,
                    SaveFile::SAVE_A_FILE_INDEX,
                    SaveFile::SAVE_A_BC_START,
                    SaveFile::SAVE_A_BC_END
                )
            }
        }
    }

    pub fn get_bc_byte_offeset(&self) -> u32 {
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
}

impl Default for SaveFile {
    fn default() -> Self {
        Self {
            label: SaveFile::SAVE_A_LABEL,
            index: SaveFile::SAVE_A_FILE_INDEX,
            bc_start_byte: SaveFile::SAVE_A_BC_START,
            bc_end_byte: SaveFile::SAVE_A_BC_END,
        }
    }
}