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
            println!("Size indicates this is not a Sunhsine save file!");
            std::process::exit(1)
        }

        // TODO: Rework this part
        //       This is to check if the uploaded save is a SMS file
        // 0x47, 0x4D, 0x53, 0x45, 0x30, 0x31
        //    G,    M,    S,    E,    0,    1
        let is_save: bool = save_file[0] == 0x47
            && save_file[1] == 0x4D
            && save_file[2] == 0x53
            && save_file[3] == 0x45
            && save_file[4] == 0x30
            && save_file[5] == 0x31;

        if !is_save {
            println!("Not a Sunshine save file!");
        }

        MemoryCard { mem_data: save_file }
    }
}