pub struct BlueCoin {
    offset_from_start: u32,
    is_collected: bool,
    mpl_order_number: u8,
    pub description: String,
}

impl BlueCoin {
    pub fn new(offset_from_start: u32, is_collected: bool, mpl_order_number: u8, description: String) -> Self {
        BlueCoin { offset_from_start, is_collected, mpl_order_number, description }
    }
}