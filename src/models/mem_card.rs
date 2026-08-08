use crate::enums::{file_error::FileError, region::Region};

#[derive(Clone)]
pub struct MemoryCard {
    pub mem_data: Vec<u8>,
    pub file_error: FileError,
    pub region: Region,
}

impl MemoryCard {
    pub fn get_mem_data(&self) -> &Vec<u8> {
        &self.mem_data
    }

    pub fn read_region(save_file: &Vec<u8>) -> Region {
        match save_file[3] {
            0x45 => Region::NTSCU,
            0x4A => Region::NTSCJ,
            0x50 => Region::PAL,
            _ => Region::INVALID
        }
    }

    fn is_game_sms(save_file: &Vec<u8>) -> bool {
        // TODO: Rework this part
        //       This is to check if the uploaded save is a SMS file
        // 0x47, 0x4D, 0x53, region, 0x30, 0x31
        //    G,    M,    S,  E|J|P,    0,    1
        return save_file[0] == 0x47
            && save_file[1] == 0x4D
            && save_file[2] == 0x53
            && !matches!(Self::read_region(save_file), Region::INVALID)
            && save_file[4] == 0x30
            && save_file[5] == 0x31
    }

    pub fn read_from_file(path: &str) -> Self {
        let sms_save_size: usize = 57408;
        let save_file: Vec<u8> = match std::fs::read(path) {
            Ok(file) => file,
            Err(_e) => {
                vec![] as Vec<u8>
            }
        };

        if save_file.len() == 0 {
            // File read error
            return MemoryCard { mem_data: vec![], file_error: FileError::FileRead, region: Region::INVALID }
        }

        if !Self::is_game_sms(&save_file) {
            // Not a sms save file
            return MemoryCard { mem_data: vec![], file_error: FileError::NotProperGame, region: Region::INVALID }
        }

        if save_file.len() != sms_save_size {
            // Length of file does not match what's expected
            return MemoryCard { mem_data: vec![], file_error: FileError::NotValidSave, region: Region::INVALID }
        }

        let region: Region = Self::read_region(&save_file);
        MemoryCard { mem_data: save_file, file_error: FileError::NoError, region }
    }
}

impl Default for MemoryCard {
    fn default() -> Self {
        Self { mem_data: vec![], file_error: FileError::NoError, region: Region::INVALID }
    }
}