pub struct MemoryCard {
    mem_data: Vec<u8>
}

impl MemoryCard {
    pub fn get_mem_data(&self) -> &Vec<u8> {
        &self.mem_data
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

        if save_file.len() != sms_save_size {
            println!("Not a proper Sunshine save file");
            std::process::exit(1)
        }

        MemoryCard { mem_data: save_file }
    }
}