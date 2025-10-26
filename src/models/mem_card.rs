use crate::enums::file_error::FileError;

#[derive(Clone)]
pub struct MemoryCard {
    pub mem_data: Vec<u8>,
    pub file_error: FileError,
}

impl MemoryCard {
    pub fn get_mem_data(&self) -> &Vec<u8> {
        &self.mem_data
    }

    fn is_region_na(save_file: &Vec<u8>) -> bool {
        // TODO: Rework this part
        //       This is to check if the game save is from N.A
        // GMSE == NA, GMSP == PAL, GMSJ == JP
        // 0x45 == E, 0x50 == P, 0x4A == J
        // This might be an overkill check, but we can narrow down
        //    the error this way.
        return save_file[3] != 0x50 
            && save_file[3] != 0x4A
            && save_file[3] == 0x45
    }

    fn is_game_sms(save_file: &Vec<u8>) -> bool {
        // TODO: Rework this part
        //       This is to check if the uploaded save is a SMS file
        // 0x47, 0x4D, 0x53, 0x45, 0x30, 0x31
        //    G,    M,    S,    E,    0,    1
        return save_file[0] == 0x47
            && save_file[1] == 0x4D
            && save_file[2] == 0x53
            && save_file[3] == 0x45
            && save_file[4] == 0x30
            && save_file[5] == 0x31
    }

    pub fn read_from_file(path: &str) -> Self {
        let sms_save_size: usize = 57408;
        let save_file: Vec<u8> = match std::fs::read(path) {
            Ok(file) => file,
            Err(e) => {
                println!("Error: {}", e);
                vec![]
            }
        };

        if save_file.len() == 0 {
            // File read error
            return MemoryCard { mem_data: vec![], file_error: FileError::FileRead }
        }

        if !Self::is_region_na(&save_file) {
            // Not North America region
            return MemoryCard { mem_data: vec![], file_error: FileError::NotProperRegion }
        }

        if !Self::is_game_sms(&save_file) {
            // Not a sms save file
            return MemoryCard { mem_data: vec![], file_error: FileError::NotProperGame }
        }

        if save_file.len() != sms_save_size {
            // Length of file does not match what's expected
            return MemoryCard { mem_data: vec![], file_error: FileError::NotValidSave }
        }

        MemoryCard { mem_data: save_file, file_error: FileError::NoError }
    }
}

impl Default for MemoryCard {
    fn default() -> Self {
        Self { mem_data: vec![], file_error: FileError::NoError }
    }
}