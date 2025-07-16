use crate::{data::data::BlueCoinData, models::{mem_card::MemoryCard, save_file::SaveFile}};

pub struct BlueCoin {
    pub is_collected: bool,
    mpl_order_number: u8,
    pub description: String,
}

impl BlueCoin {
    pub fn new(is_collected: bool, mpl_order_number: u8, description: String) -> Self {
        BlueCoin { is_collected, mpl_order_number, description }
    }

    fn get_blue_coin_collected(mem_card: &MemoryCard, save_file: &SaveFile, bcd: &BlueCoinData) -> bool {//address: usize, index: u32) -> bool {
        // Blue coin flags are stored in 2 addresses:
        // 0x2nnn and 0x4nnn (for File A)
        // The game switches between the two every save and will NOT save at both at the same time
        // The game will always save at the 0x2nnn address first
        // Ex: Get H.S, save -> Addr 1. Save again -> Addr 2. Save a 3rd time -> Addr 1. etc...
        // We need to check which address has the flag enabled (if it is at all)

        let save_file_offset: u32 = save_file.get_bc_byte_offeset();
        let address: u32 = bcd.get_byte() + save_file_offset;
        let index: u32 = bcd.get_byte_index();

        let flag = mem_card.sms_save_memory[address as usize] | mem_card.sms_save_memory[(address + 0x2000) as usize];
        (flag & 2_u8.pow(index)) != 0
    }

    pub fn from_data(mem_card: &MemoryCard, save_file: &SaveFile, data: &BlueCoinData) -> BlueCoin {
        BlueCoin::new(
            Self::get_blue_coin_collected(mem_card, save_file, data),
            data.get_mpl_order(),
            data.get_description()
        )
    }
}