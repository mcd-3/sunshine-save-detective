use crate::{
    models::{
        mem_card::MemoryCard,
        save_file::SaveFile
    },
    data::nozzle_data::NozzleData,
    enums::nozzle_type::NozzleType,
};

pub struct Nozzle {
    is_collected: bool,
    nozzle_type: NozzleType
}

impl Nozzle {
    pub fn new(is_collected: bool, nozzle_type: NozzleType) -> Self {
        Nozzle { is_collected, nozzle_type }
    }

    pub fn get_is_collected(&self) -> bool {
        self.is_collected
    }

    pub fn get_nozzle_name(&self) -> String {
        match self.nozzle_type {
            NozzleType::Rocket => String::from("Rocket Nozzle"),
            NozzleType::Turbo => String::from("Turbo Nozzle"),
        }
    }

    pub fn get_overview(&self) -> String {
        let nozzle_str = match self.nozzle_type {
            NozzleType::Rocket => "Rocket Nozzle",
            NozzleType::Turbo => "Turbo Nozzle"
        };

        return format!(
            "{}\n\n{}",
            nozzle_str,
            if self.is_collected { "Collected" } else { "Missing" }
        )
    }

    fn get_nozzle_collected(mem_card: &MemoryCard, save_file: &SaveFile, nd: &NozzleData) -> bool {
        // Blue coin flags are stored in 2 addresses:
        // 0x2nnn and 0x4nnn (for File A)
        // The game switches between the two every save and will NOT save at both at the same time
        // The game will always save at the 0x2nnn address first
        // Ex: Get H.S, save -> Addr 1. Save again -> Addr 2. Save a 3rd time -> Addr 1. etc...
        // We need to check which address has the flag enabled (if it is at all)

        let save_file_offset: u32 = save_file.get_data_start_offeset();
        let address: u32 = nd.get_byte() + save_file_offset;
        let index: u32 = nd.get_byte_index();

        let flag: u8 = mem_card.get_mem_data()[address as usize] | mem_card.get_mem_data()[(address + 0x2000) as usize];
        (flag & 2_u8.pow(index)) != 0
    }

    pub fn from_data(mem_card: &MemoryCard, save_file: &SaveFile, data: &NozzleData) -> Nozzle {
        Nozzle::new(
            Self::get_nozzle_collected(mem_card, save_file, data),
            data.get_nozzle_type(),
        )
    }
}