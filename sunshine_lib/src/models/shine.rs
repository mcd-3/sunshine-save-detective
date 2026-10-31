use crate::{
    models::{
        save_file::SaveFile,
        mem_card::MemoryCard
    },
    data::shine_data::ShineData,
};

#[derive(Clone)]
pub struct Shine {
    is_collected: bool,
    description: String,
    is_secret: bool,
}

impl Shine {
    pub fn new(is_collected: bool, description: String, is_secret: bool) -> Self {
        Shine { is_collected, description, is_secret }
    }

    pub fn get_is_collected(&self) -> bool {
        self.is_collected
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }

    pub fn get_is_secret(&self) -> bool {
        self.is_secret
    }

    pub fn get_overview(&self) -> String {
        return format!(
            "{}\n\n{}",
            self.get_description(),
            if self.is_collected { "Collected" } else { "Missing" }
        )
    }

    fn get_shine_collected(mem_card: &MemoryCard, save_file: &SaveFile, sd: &ShineData) -> bool {
        // Shine flags are stored in 2 addresses:
        // 0x2nnn and 0x4nnn (for File A)
        // The game switches between the two every save and will NOT save at both at the same time
        // The game will always save at the 0x2nnn address first
        // Ex: Get H.S, save -> Addr 1. Save again -> Addr 2. Save a 3rd time -> Addr 1. etc...
        // We need to check which address has the flag enabled (if it is at all)

        let save_file_offset: u32 = save_file.get_data_start_offeset();
        let address: u32 = sd.get_byte() + save_file_offset;
        let index: u32 = sd.get_byte_index();

        let flag: u8 = mem_card.get_mem_data()[address as usize] | mem_card.get_mem_data()[(address + 0x2000) as usize];
        (flag & 2_u8.pow(index)) != 0
    }

    pub fn from_data(mem_card: &MemoryCard, save_file: &SaveFile, data: &ShineData) -> Shine {
        Shine::new(
            Self::get_shine_collected(mem_card, save_file, data),
            data.get_description(),
            data.get_is_secret()
        )
    }
}