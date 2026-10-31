pub struct BlueCoinData {
    byte: u32,
    bit_index: u32,
    mpl_order: u8,
    description: String,
}

impl BlueCoinData {
    // Indexes of binary values
    // Ex: 00000001 = 1 spot = 0, 00100000 = 32 spot = 5, etc...
    const BINARY_INDEX_1: u32 = 0;
    const BINARY_INDEX_2: u32 = 1;
    const BINARY_INDEX_4: u32 = 2;
    const BINARY_INDEX_8: u32 = 3;
    const BINARY_INDEX_16: u32 = 4;
    const BINARY_INDEX_32: u32 = 5;
    const BINARY_INDEX_64: u32 = 6;
    const BINARY_INDEX_128: u32 = 7;

    pub fn new(byte: u32, bit_index: u32, mpl_order: u8, description: String) -> BlueCoinData {
        BlueCoinData { byte, bit_index, description, mpl_order }
    }

    pub fn get_byte(&self) -> u32 {
        self.byte
    }

    pub fn get_byte_index(&self) -> u32 {
        self.bit_index
    }

    pub fn get_mpl_order(&self) -> u8 {
        self.mpl_order
    }

    pub fn get_description(&self) -> String {
        self.description.clone()
    }

    pub fn get_delfino_bcs() -> Vec<BlueCoinData> {
        vec![
            BlueCoinData::new(0x00AF, Self::BINARY_INDEX_128, 1, String::from("'M' hidden by crate near the pinna park cannon")),
            BlueCoinData::new(0x00AF, Self::BINARY_INDEX_64, 2, String::from("'M' on the tower near the pinna park cannon")),
            BlueCoinData::new(0x00AF, Self::BINARY_INDEX_32, 10, String::from("'M' on the shine gate ")),
            BlueCoinData::new(0x00AF, Self::BINARY_INDEX_16, 16, String::from("Pianta on fire")),
            BlueCoinData::new(0x00AF, Self::BINARY_INDEX_8, 3, String::from("Spray the 'X' near the water")),
            BlueCoinData::new(0x00AF, Self::BINARY_INDEX_4, 4, String::from("Spray the 'X' near Bianco Hills' entrance")),
            BlueCoinData::new(0x00AF, Self::BINARY_INDEX_2, 17, String::from("Behind gate near the rocks")),
            BlueCoinData::new(0x00AF, Self::BINARY_INDEX_1, 20, String::from("Ice cube in Delfino Airstrip")),

            BlueCoinData::new(0x00B0, Self::BINARY_INDEX_128, 18, String::from("Behind yellow gunk near Gelato Beach's entrance")),
            BlueCoinData::new(0x00B0, Self::BINARY_INDEX_64, 11, String::from("In the jail")),
            BlueCoinData::new(0x00B0, Self::BINARY_INDEX_32, 19, String::from("Behind yellow gunk near Pinna Park's entrance")),
            BlueCoinData::new(0x00B0, Self::BINARY_INDEX_16, 7, String::from("Underground near the island")),
            BlueCoinData::new(0x00B0, Self::BINARY_INDEX_8, 15, String::from("Coconut fruit basket")),
            BlueCoinData::new(0x00B0, Self::BINARY_INDEX_4, 12, String::from("Banana fruit basket")),
            BlueCoinData::new(0x00B0, Self::BINARY_INDEX_2, 14, String::from("Durian fruit basket")),
            BlueCoinData::new(0x00B0, Self::BINARY_INDEX_1, 13, String::from("Pineapple fruit basket")),

            BlueCoinData::new(0x00B1, Self::BINARY_INDEX_1, 6, String::from("Blue bird near the crate smash building")),
            BlueCoinData::new(0x00B1, Self::BINARY_INDEX_2, 5, String::from("Blue bird near Serena Beach's entrance")),
            BlueCoinData::new(0x00B1, Self::BINARY_INDEX_4, 8, String::from("Underground near the crate smash building")),
            BlueCoinData::new(0x00B1, Self::BINARY_INDEX_8, 9, String::from("Underground near Corona Mountain")),
        ]
    }

    pub fn get_bianco_bcs() -> Vec<BlueCoinData> {
        vec![
            BlueCoinData::new(0x00B5, Self::BINARY_INDEX_128, 14, String::from("Pianta in cavity")),
            BlueCoinData::new(0x00B5, Self::BINARY_INDEX_64, 17, String::from("Palm tree near ropes")),
            BlueCoinData::new(0x00B5, Self::BINARY_INDEX_32, 3, String::from("Spray water in right doorway of the rocket nozzle building in the village")),
            BlueCoinData::new(0x00B5, Self::BINARY_INDEX_16, 2, String::from("Building with 2 towers")),
            BlueCoinData::new(0x00B5, Self::BINARY_INDEX_8, 12, String::from("Spray stone pillar near large windmill")),
            BlueCoinData::new(0x00B5, Self::BINARY_INDEX_4, 11, String::from("'M' on big windmill path")),

            BlueCoinData::new(0x00B6, Self::BINARY_INDEX_128, 19, String::from("Under chain bridge")),
            BlueCoinData::new(0x00B6, Self::BINARY_INDEX_64, 24, String::from("Blue bird in village")),
            BlueCoinData::new(0x00B6, Self::BINARY_INDEX_32, 28, String::from("Underwater in windmill lake")),
            BlueCoinData::new(0x00B6, Self::BINARY_INDEX_16, 13, String::from("Underwater in windmill lake")),
            BlueCoinData::new(0x00B6, Self::BINARY_INDEX_8, 23, String::from("Past the cloud platforms")),
            BlueCoinData::new(0x00B6, Self::BINARY_INDEX_4, 20, String::from("Hillside brick platform Pokey")),
            BlueCoinData::new(0x00B6, Self::BINARY_INDEX_2, 21, String::from("Border wall pianta")),
            BlueCoinData::new(0x00B6, Self::BINARY_INDEX_1, 30, String::from("Beehive near chain bridge")),

            BlueCoinData::new(0x00B7, Self::BINARY_INDEX_128, 1, String::from("Underwater near the dam")),
            BlueCoinData::new(0x00B7, Self::BINARY_INDEX_64, 9, String::from("'X' near lakeside wall")),
            BlueCoinData::new(0x00B7, Self::BINARY_INDEX_32, 10, String::from("Spray wind wheel")),
            BlueCoinData::new(0x00B7, Self::BINARY_INDEX_16, 5, String::from("'M' behind 2 tower house")),
            BlueCoinData::new(0x00B7, Self::BINARY_INDEX_8, 7, String::from("'M' on top of village border wall")),
            BlueCoinData::new(0x00B7, Self::BINARY_INDEX_4, 6, String::from("'M' behind waterwheel wall")),
            BlueCoinData::new(0x00B7, Self::BINARY_INDEX_2, 22, String::from("Petey Pianta wooden platform")),
            BlueCoinData::new(0x00B7, Self::BINARY_INDEX_1, 16, String::from("On top of large windmill")),

            BlueCoinData::new(0x00B8, Self::BINARY_INDEX_128, 15, String::from("Tallest wooden pillar")),
            BlueCoinData::new(0x00B8, Self::BINARY_INDEX_64, 4, String::from("Spinning bridge")),
            BlueCoinData::new(0x00B8, Self::BINARY_INDEX_32, 8, String::from("'X' in wall cavity")),
            BlueCoinData::new(0x00B8, Self::BINARY_INDEX_16, 29, String::from("Blue butterflies")),
            BlueCoinData::new(0x00B8, Self::BINARY_INDEX_8, 27, String::from("Circle paint near river")),
            BlueCoinData::new(0x00B8, Self::BINARY_INDEX_4, 26, String::from("Circle paint near throwing Pianta")),
            BlueCoinData::new(0x00B8, Self::BINARY_INDEX_2, 25, String::from("Throwing Pianta")),
            BlueCoinData::new(0x00B8, Self::BINARY_INDEX_1, 18, String::from("Ledge near village")),
        ]
    }

    pub fn get_ricco_bcs() -> Vec<BlueCoinData> {
        vec![
            BlueCoinData::new(0x00BB, Self::BINARY_INDEX_128, 2, String::from("'M' on top of warehouse")),
            BlueCoinData::new(0x00BB, Self::BINARY_INDEX_64, 3, String::from("Ship side 'M'")),
            BlueCoinData::new(0x00BB, Self::BINARY_INDEX_32, 5, String::from("Hidden shine outline")),
            BlueCoinData::new(0x00BB, Self::BINARY_INDEX_16, 29, String::from("Blue butterflies")),

            BlueCoinData::new(0x00BC, Self::BINARY_INDEX_128, 15, String::from("Blue railing near town")),
            BlueCoinData::new(0x00BC, Self::BINARY_INDEX_64, 6, String::from("Tower scaffolding")),
            BlueCoinData::new(0x00BC, Self::BINARY_INDEX_32, 14, String::from("Blue railing")),
            BlueCoinData::new(0x00BC, Self::BINARY_INDEX_16, 18, String::from("Palm trees 'M'")),
            BlueCoinData::new(0x00BC, Self::BINARY_INDEX_8, 21, String::from("Tower floor 'M'")),
            BlueCoinData::new(0x00BC, Self::BINARY_INDEX_4, 9, String::from("Roof of town building 'M'")),
            BlueCoinData::new(0x00BC, Self::BINARY_INDEX_2, 30, String::from("Eat wall Klamber with Yoshi")),
            BlueCoinData::new(0x00BC, Self::BINARY_INDEX_1, 11, String::from("Building 'X'")),

            BlueCoinData::new(0x00BD, Self::BINARY_INDEX_128, 10, String::from("Tower 'X'")),
            BlueCoinData::new(0x00BD, Self::BINARY_INDEX_64, 13, String::from("Underwater")),
            BlueCoinData::new(0x00BD, Self::BINARY_INDEX_32, 7, String::from("Water fountain")),
            BlueCoinData::new(0x00BD, Self::BINARY_INDEX_16, 23, String::from("Water jump with blooper near tunnel")),
            BlueCoinData::new(0x00BD, Self::BINARY_INDEX_8, 12, String::from("Water jump with nozzle/blooper")),
            BlueCoinData::new(0x00BD, Self::BINARY_INDEX_4, 17, String::from("Moving crane")),
            BlueCoinData::new(0x00BD, Self::BINARY_INDEX_2, 26, String::from("Rocket nozzle tower")),
            BlueCoinData::new(0x00BD, Self::BINARY_INDEX_1, 16, String::from("Fall from blue railing")),

            BlueCoinData::new(0x00BE, Self::BINARY_INDEX_128, 27, String::from("Yellow cage blooper")),
            BlueCoinData::new(0x00BE, Self::BINARY_INDEX_64, 20, String::from("Cargo ship 'M'")),
            BlueCoinData::new(0x00BE, Self::BINARY_INDEX_32, 28, String::from("Fish bucket")),
            BlueCoinData::new(0x00BE, Self::BINARY_INDEX_16, 24, String::from("Klamber spider in ship")),
            BlueCoinData::new(0x00BE, Self::BINARY_INDEX_8, 25, String::from("Klamber spider in ship #2")),
            BlueCoinData::new(0x00BE, Self::BINARY_INDEX_4, 19, String::from("Yoshi adventure 'M'")),
            BlueCoinData::new(0x00BE, Self::BINARY_INDEX_2, 1, String::from("Warehouse 'M'")),
            BlueCoinData::new(0x00BE, Self::BINARY_INDEX_1, 8, String::from("Town building 'M'")),

            BlueCoinData::new(0x00BF, Self::BINARY_INDEX_2, 22, String::from("Yellow submarine turbine")),
            BlueCoinData::new(0x00BF, Self::BINARY_INDEX_1, 4, String::from("Tower crate")),
        ]
    }

    pub fn get_gelato_bcs() -> Vec<BlueCoinData> {
        vec![
            BlueCoinData::new(0x00C1, Self::BINARY_INDEX_128, 17, String::from("Sand shine outline #3")),
            BlueCoinData::new(0x00C1, Self::BINARY_INDEX_64, 22, String::from("Red Cataquack")),

            BlueCoinData::new(0x00C2, Self::BINARY_INDEX_128, 29, String::from("Yellow goop with yoshi")),
            BlueCoinData::new(0x00C2, Self::BINARY_INDEX_64, 12, String::from("Triangle near surf cabana")),
            BlueCoinData::new(0x00C2, Self::BINARY_INDEX_32, 13, String::from("Triangle near fruit cabana")),
            BlueCoinData::new(0x00C2, Self::BINARY_INDEX_16, 4, String::from("'M' on rock wall")),
            BlueCoinData::new(0x00C2, Self::BINARY_INDEX_8, 10, String::from("Fruit grinder")),
            BlueCoinData::new(0x00C2, Self::BINARY_INDEX_4, 16, String::from("Sand shine outline #2")),
            BlueCoinData::new(0x00C2, Self::BINARY_INDEX_2, 24, String::from("Palm tree near big mirrors")),
            BlueCoinData::new(0x00C2, Self::BINARY_INDEX_1, 15, String::from("Sand shine outline")),

            BlueCoinData::new(0x00C3, Self::BINARY_INDEX_128, 21, String::from("School of fish #2")),
            BlueCoinData::new(0x00C3, Self::BINARY_INDEX_64, 20, String::from("School of fish")),
            BlueCoinData::new(0x00C3, Self::BINARY_INDEX_32, 19, String::from("In the ocean #2")),
            BlueCoinData::new(0x00C3, Self::BINARY_INDEX_16, 18, String::from("In the ocean")),
            BlueCoinData::new(0x00C3, Self::BINARY_INDEX_8, 7, String::from("Top of mountain pole")),
            BlueCoinData::new(0x00C3, Self::BINARY_INDEX_4, 9, String::from("Ropes of large poles near top of mountain")),
            BlueCoinData::new(0x00C3, Self::BINARY_INDEX_2, 5, String::from("Blue bird at top of mountain")),
            BlueCoinData::new(0x00C3, Self::BINARY_INDEX_1, 6, String::from("Blue bird at top of mountain #2")),

            BlueCoinData::new(0x00C4, Self::BINARY_INDEX_128, 30, String::from("Beehive with Yoshi")),
            BlueCoinData::new(0x00C4, Self::BINARY_INDEX_64, 14, String::from("Rocket near large mirrors")),
            BlueCoinData::new(0x00C4, Self::BINARY_INDEX_32, 3, String::from("Sand cabana")),
            BlueCoinData::new(0x00C4, Self::BINARY_INDEX_16, 23, String::from("Big sand shine outline")),
            BlueCoinData::new(0x00C4, Self::BINARY_INDEX_8, 8, String::from("Gap in path of mountain")),
            BlueCoinData::new(0x00C4, Self::BINARY_INDEX_4, 1, String::from("Top of swing palm tree")),
            BlueCoinData::new(0x00C4, Self::BINARY_INDEX_2, 2, String::from("Swing from tree")),
            BlueCoinData::new(0x00C4, Self::BINARY_INDEX_1, 11, String::from("Under surf cabana in water")),

            BlueCoinData::new(0x00C5, Self::BINARY_INDEX_8, 25, String::from("Sand bird")),
            BlueCoinData::new(0x00C5, Self::BINARY_INDEX_4, 26, String::from("Sand bird #2")),
            BlueCoinData::new(0x00C5, Self::BINARY_INDEX_2, 27, String::from("Sand bird #3")),
            BlueCoinData::new(0x00C5, Self::BINARY_INDEX_1, 28, String::from("Sand bird #4")),
        ]
    }

    pub fn get_pinna_bcs() -> Vec<BlueCoinData> {
        vec![
            BlueCoinData::new(0x00C8, Self::BINARY_INDEX_128, 25, String::from("Wood ledge near atop of ships")),
            BlueCoinData::new(0x00C8, Self::BINARY_INDEX_64, 23, String::from("Wood ledge near green metal path")),
            BlueCoinData::new(0x00C8, Self::BINARY_INDEX_32, 17, String::from("Blue bird on mini gazebo")),
            BlueCoinData::new(0x00C8, Self::BINARY_INDEX_16, 11, String::from("Blue bird in park")),
            BlueCoinData::new(0x00C8, Self::BINARY_INDEX_8, 20, String::from("Circle at top of ferris wheel")),
            BlueCoinData::new(0x00C8, Self::BINARY_INDEX_4, 12, String::from("In clam ride")),
            BlueCoinData::new(0x00C8, Self::BINARY_INDEX_2, 28, String::from("'M' on stone wall")),
            BlueCoinData::new(0x00C8, Self::BINARY_INDEX_1, 14, String::from("'M' on orange arch")),

            BlueCoinData::new(0x0C9, Self::BINARY_INDEX_128, 30, String::from("Blue butterfly in park near palm tree")),
            BlueCoinData::new(0x0C9, Self::BINARY_INDEX_64, 13, String::from("In metal column")),
            BlueCoinData::new(0x0C9, Self::BINARY_INDEX_32, 18, String::from("Triangle on brick wall")),
            BlueCoinData::new(0x0C9, Self::BINARY_INDEX_16, 19, String::from("Triangle on ledge near banana")),
            BlueCoinData::new(0x0C9, Self::BINARY_INDEX_8, 29, String::from("'M' behind the ferris wheel")),
            BlueCoinData::new(0x0C9, Self::BINARY_INDEX_4, 15, String::from("'X' on pianta tree")),
            BlueCoinData::new(0x0C9, Self::BINARY_INDEX_2, 16, String::from("'X' on white wall")),
            BlueCoinData::new(0x0C9, Self::BINARY_INDEX_1, 21, String::from("Circle at top of ferris wheel #2")),

            BlueCoinData::new(0x00CA, Self::BINARY_INDEX_128, 9, String::from("Blue butterfly on beach")),
            BlueCoinData::new(0x00CA, Self::BINARY_INDEX_64, 6, String::from("Basket on beach #4")),
            BlueCoinData::new(0x00CA, Self::BINARY_INDEX_32, 8, String::from("Basket on beach #6")),
            BlueCoinData::new(0x00CA, Self::BINARY_INDEX_16, 7, String::from("Basket on beach #5")),
            BlueCoinData::new(0x00CA, Self::BINARY_INDEX_8, 27, String::from("Strollin' Stu tower")),
            BlueCoinData::new(0x00CA, Self::BINARY_INDEX_4, 26, String::from("Green metal box near ships")),
            BlueCoinData::new(0x00CA, Self::BINARY_INDEX_2, 22, String::from("Grass ledge near ships")),
            BlueCoinData::new(0x00CA, Self::BINARY_INDEX_1, 24, String::from("Green metal pole near ships")),

            BlueCoinData::new(0x00CB, Self::BINARY_INDEX_32, 2, String::from("Shine outline on beach #2")),
            BlueCoinData::new(0x00CB, Self::BINARY_INDEX_16, 1, String::from("Shine outline on beach")),
            BlueCoinData::new(0x00CB, Self::BINARY_INDEX_8, 3, String::from("Basket on beach")),
            BlueCoinData::new(0x00CB, Self::BINARY_INDEX_4, 4, String::from("Basket on beach #2")),
            BlueCoinData::new(0x00CB, Self::BINARY_INDEX_2, 5, String::from("Basket on beach #3")),
            BlueCoinData::new(0x00CB, Self::BINARY_INDEX_1, 10, String::from("Blue butterfly on beach #2")),
        ]
    }

    pub fn get_sirena_bcs() -> Vec<BlueCoinData> {
        vec![
            BlueCoinData::new(0x00CE, Self::BINARY_INDEX_128, 5, String::from("Behind the hotel")),
            BlueCoinData::new(0x00CE, Self::BINARY_INDEX_64, 10, String::from("Spray the Nokki #4")),
            BlueCoinData::new(0x00CE, Self::BINARY_INDEX_32, 9, String::from("Spray the Nokki #3")),
            BlueCoinData::new(0x00CE, Self::BINARY_INDEX_16, 4, String::from("Spray the torch outside")),
            BlueCoinData::new(0x00CE, Self::BINARY_INDEX_8, 2, String::from("Smoothie shack umbrella")),
            BlueCoinData::new(0x00CE, Self::BINARY_INDEX_4, 6, String::from("Spray the fish tablet")),

            BlueCoinData::new(0x00CF, Self::BINARY_INDEX_128, 23, String::from("Attic maze")),
            BlueCoinData::new(0x00CF, Self::BINARY_INDEX_64, 17, String::from("Bookshelves in storage room")),
            BlueCoinData::new(0x00CF, Self::BINARY_INDEX_32, 18, String::from("Behind Boo poster in storage room")),
            BlueCoinData::new(0x00CF, Self::BINARY_INDEX_16, 21, String::from("Spray the mirror in a room")),
            BlueCoinData::new(0x00CF, Self::BINARY_INDEX_8, 22, String::from("Groundpound glass table in room with mirror")),
            BlueCoinData::new(0x00CF, Self::BINARY_INDEX_4, 20, String::from("Room under storage room")),
            BlueCoinData::new(0x00CF, Self::BINARY_INDEX_2, 19, String::from("Box under storage room")),
            BlueCoinData::new(0x00CF, Self::BINARY_INDEX_1, 16, String::from("Spray the large ceiling light")),

            BlueCoinData::new(0x00D0, Self::BINARY_INDEX_128, 14, String::from("'M' in casino")),
            BlueCoinData::new(0x00D0, Self::BINARY_INDEX_64, 7, String::from("Spray the Nokki")),
            BlueCoinData::new(0x00D0, Self::BINARY_INDEX_32, 30, String::from("'M' on third floor")),
            BlueCoinData::new(0x00D0, Self::BINARY_INDEX_16, 11, String::from("'M' near hotel outside")),
            BlueCoinData::new(0x00D0, Self::BINARY_INDEX_8, 1, String::from("In the ocean")),
            BlueCoinData::new(0x00D0, Self::BINARY_INDEX_4, 3, String::from("Flower bed outside")),
            BlueCoinData::new(0x00D0, Self::BINARY_INDEX_2, 24, String::from("Boo in attic")),
            BlueCoinData::new(0x00D0, Self::BINARY_INDEX_1, 8, String::from("Spray the Nokki #2")),

            BlueCoinData::new(0x00D1, Self::BINARY_INDEX_128, 13, String::from("Groundpound slot machine")),
            BlueCoinData::new(0x00D1, Self::BINARY_INDEX_64, 12, String::from("Spray the right-side torch in the casino")),
            BlueCoinData::new(0x00D1, Self::BINARY_INDEX_32, 28, String::from("'X' on second floor")),
            BlueCoinData::new(0x00D1, Self::BINARY_INDEX_16, 29, String::from("'X' on third floor")),
            BlueCoinData::new(0x00D1, Self::BINARY_INDEX_8, 25, String::from("'M' in attic")),
            BlueCoinData::new(0x00D1, Self::BINARY_INDEX_4, 27, String::from("Triangle on second floor")),
            BlueCoinData::new(0x00D1, Self::BINARY_INDEX_2, 26, String::from("Triangle on first floor")),
            BlueCoinData::new(0x00D1, Self::BINARY_INDEX_1, 15, String::from("Spray the lamp")),
        ]
    }

    pub fn get_noki_bcs() -> Vec<BlueCoinData> {
        vec![
            BlueCoinData::new(0x00DA, Self::BINARY_INDEX_128, 10, String::from("Gooper Blooper wall maze")),
            BlueCoinData::new(0x00DA, Self::BINARY_INDEX_64, 5, String::from("Hole in wall next to rocket nozzle")),

            BlueCoinData::new(0x00DB, Self::BINARY_INDEX_128, 19, String::from("8 tiles")),
            BlueCoinData::new(0x00DB, Self::BINARY_INDEX_64, 15, String::from("Rocket to hole in cliff wall")),
            BlueCoinData::new(0x00DB, Self::BINARY_INDEX_32, 9, String::from("Hole in wall near top of waterfall")),
            BlueCoinData::new(0x00DB, Self::BINARY_INDEX_16, 7, String::from("Wall jump in wall hole near vase")),
            BlueCoinData::new(0x00DB, Self::BINARY_INDEX_8, 8, String::from("Shimmy near vase")),
            BlueCoinData::new(0x00DB, Self::BINARY_INDEX_4, 6, String::from("Hole in wall near vase")),
            BlueCoinData::new(0x00DB, Self::BINARY_INDEX_2, 4, String::from("Rocket nozzle near cliffs")),
            BlueCoinData::new(0x00DB, Self::BINARY_INDEX_1, 11, String::from("Gooper Blooper wall maze #2")),

            BlueCoinData::new(0x00DC, Self::BINARY_INDEX_128, 17, String::from("2 wall holes")),
            BlueCoinData::new(0x00DC, Self::BINARY_INDEX_64, 18, String::from("Below 2 wall holes")),
            BlueCoinData::new(0x00DC, Self::BINARY_INDEX_32, 23, String::from("Vertical long ledge")),
            BlueCoinData::new(0x00DC, Self::BINARY_INDEX_16, 21, String::from("Tunnel with 2 tiles")),
            BlueCoinData::new(0x00DC, Self::BINARY_INDEX_8, 22, String::from("Small ledge with 2 tiles")),
            BlueCoinData::new(0x00DC, Self::BINARY_INDEX_4, 24, String::from("Single hole in the wall")),
            BlueCoinData::new(0x00DC, Self::BINARY_INDEX_2, 16, String::from("Rocket to hole in cliff wall #2")),
            BlueCoinData::new(0x00DC, Self::BINARY_INDEX_1, 20, String::from("8 tiles #2")),

            BlueCoinData::new(0x00DD, Self::BINARY_INDEX_128, 28, String::from("Underwater city #2")),
            BlueCoinData::new(0x00DD, Self::BINARY_INDEX_64, 25, String::from("Circle on wall near start")),
            BlueCoinData::new(0x00DD, Self::BINARY_INDEX_32, 14, String::from("'M' on wall near Glooper Blooper path")),
            BlueCoinData::new(0x00DD, Self::BINARY_INDEX_16, 3, String::from("Deep under the water near waterfall")),
            BlueCoinData::new(0x00DD, Self::BINARY_INDEX_8, 2, String::from("Above the water #2")),
            BlueCoinData::new(0x00DD, Self::BINARY_INDEX_4, 1, String::from("Above the water")),
            BlueCoinData::new(0x00DD, Self::BINARY_INDEX_2, 13, String::from("King's tomb vase #2")),
            BlueCoinData::new(0x00DD, Self::BINARY_INDEX_1, 12, String::from("King's tomb vase")),

            BlueCoinData::new(0x00DE, Self::BINARY_INDEX_8, 30, String::from("Underwater city #4")),
            BlueCoinData::new(0x00DE, Self::BINARY_INDEX_4, 26, String::from("Circle near boat hut")),
            BlueCoinData::new(0x00DE, Self::BINARY_INDEX_2, 29, String::from("Underwater city #3")),
            BlueCoinData::new(0x00DE, Self::BINARY_INDEX_1, 27, String::from("Underwater city")),
        ]
    }

    pub fn get_pianta_bcs() -> Vec<BlueCoinData> {
        vec![
            BlueCoinData::new(0x00D4, Self::BINARY_INDEX_128, 29, String::from("Clean up pianta #8")),
            BlueCoinData::new(0x00D4, Self::BINARY_INDEX_64, 27, String::from("Clean up pianta #6")),
            BlueCoinData::new(0x00D4, Self::BINARY_INDEX_32, 22, String::from("Clean up pianta")),
            BlueCoinData::new(0x00D4, Self::BINARY_INDEX_16, 13, String::from("Spray the moon")),

            BlueCoinData::new(0x00D5, Self::BINARY_INDEX_128, 15, String::from("Spray a signpost")),
            BlueCoinData::new(0x00D5, Self::BINARY_INDEX_64, 17, String::from("Giant 'M' under lava goop")),
            BlueCoinData::new(0x00D5, Self::BINARY_INDEX_32, 14, String::from("Groundpound pianta statue's nose")),
            BlueCoinData::new(0x00D5, Self::BINARY_INDEX_16, 23, String::from("Clean up pianta #2")),
            BlueCoinData::new(0x00D5, Self::BINARY_INDEX_8, 24, String::from("Clean up pianta #3")),
            BlueCoinData::new(0x00D5, Self::BINARY_INDEX_4, 25, String::from("Clean up pianta #4")),
            BlueCoinData::new(0x00D5, Self::BINARY_INDEX_2, 26, String::from("Clean up pianta #5")),
            BlueCoinData::new(0x00D5, Self::BINARY_INDEX_1, 28, String::from("Clean up pianta #7")),

            BlueCoinData::new(0x00D6, Self::BINARY_INDEX_128, 6, String::from("Waterfall bars")),
            BlueCoinData::new(0x00D6, Self::BINARY_INDEX_64, 12, String::from("Palm tree platform")),
            BlueCoinData::new(0x00D6, Self::BINARY_INDEX_32, 19, String::from("Yoshi bees near bridge")),
            BlueCoinData::new(0x00D6, Self::BINARY_INDEX_16, 20, String::from("Yoshi bees near pool")),
            BlueCoinData::new(0x00D6, Self::BINARY_INDEX_8, 5, String::from("Water stream")),
            BlueCoinData::new(0x00D6, Self::BINARY_INDEX_4, 9, String::from("Top of giant palm tree")),
            BlueCoinData::new(0x00D6, Self::BINARY_INDEX_2, 11, String::from("In the grass")),
            BlueCoinData::new(0x00D6, Self::BINARY_INDEX_1, 4, String::from("End of water steam")),

            BlueCoinData::new(0x00D7, Self::BINARY_INDEX_128, 1, String::from("Bridge 'M'")),
            BlueCoinData::new(0x00D7, Self::BINARY_INDEX_64, 3, String::from("'M' near pond")),
            BlueCoinData::new(0x00D7, Self::BINARY_INDEX_32, 10, String::from("'M' below fruit tree")),
            BlueCoinData::new(0x00D7, Self::BINARY_INDEX_16, 8, String::from("Brick wall triangle")),
            BlueCoinData::new(0x00D7, Self::BINARY_INDEX_8, 7, String::from("Rock wall triangle")),
            BlueCoinData::new(0x00D7, Self::BINARY_INDEX_4, 18, String::from("Pianta on fire")),
            BlueCoinData::new(0x00D7, Self::BINARY_INDEX_2, 30, String::from("Blue bird near big mushroom")),
            BlueCoinData::new(0x00D7, Self::BINARY_INDEX_1, 21, String::from("Blue butterflies with Yoshi")),

            BlueCoinData::new(0x00D8, Self::BINARY_INDEX_2, 16, String::from("'M' near golden mushroom")),
            BlueCoinData::new(0x00D8, Self::BINARY_INDEX_1, 2, String::from("Under the village 'M'")),
        ]
    }

    pub fn get_corona_bcs() -> Vec<BlueCoinData> {
        vec![
            BlueCoinData::new(0x00E3, Self::BINARY_INDEX_128, 10, String::from("In lava near clouds #9")),
            BlueCoinData::new(0x00E3, Self::BINARY_INDEX_64, 3, String::from("In lava near clouds #2")),
            BlueCoinData::new(0x00E3, Self::BINARY_INDEX_32, 6, String::from("In lava near clouds #5")),
            BlueCoinData::new(0x00E3, Self::BINARY_INDEX_16, 1, String::from("Stone platform")),

            BlueCoinData::new(0x00E4, Self::BINARY_INDEX_32, 8, String::from("In lava near clouds #7")),
            BlueCoinData::new(0x00E4, Self::BINARY_INDEX_16, 7, String::from("In lava near clouds #6")),
            BlueCoinData::new(0x00E4, Self::BINARY_INDEX_8, 5, String::from("In lava near clouds #4")),
            BlueCoinData::new(0x00E4, Self::BINARY_INDEX_4, 4, String::from("In lava near clouds #3")),
            BlueCoinData::new(0x00E4, Self::BINARY_INDEX_2, 9, String::from("In lava near clouds #8")),
            BlueCoinData::new(0x00E4, Self::BINARY_INDEX_1, 2, String::from("In lava near clouds")),
        ]
    }
}