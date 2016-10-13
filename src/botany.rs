use biome::Biome;
//
// Should silk be treated as a plant?
//

pub enum Plant {
    Maize,
    Barley,
    Wheat,
    Rye,
    Rice,
    Beans,
    Coral,
    Seaweed,
    Algae,
    SugarCane,
    Potato,
    Grape,
    Coffee,
    Olive,
    Rapeseed,
    Chili,
    Pepper,
    Tea,
    Citrus,
    Rubber,
    Yam,
    Cacao,
    SugarBeet,
    Aspargus,
    Coconut,
    Carrot,
    Almond,
    Banana,
    Cotton,
    SoyBeans,
    Coca,
    Kale,
    Broccoli,
    Oak,
}

pub fn get_plants(plant_set: u8, biome: Biome) -> Vec<Plant> {
    match biome {
        _ => Vec::new()
    }
}

pub fn byte_to_bitarray(byte: u8) -> [bool; 8] {
    [(byte >> 7 & 1) == 1,
     (byte >> 6 & 1) == 1,
     (byte >> 5 & 1) == 1,
     (byte >> 4 & 1) == 1,
     (byte >> 3 & 1) == 1,
     (byte >> 2 & 1) == 1,
     (byte >> 1 & 1) == 1,
     (byte      & 1) == 1]
}

pub fn bitarray_to_byte(bool_vec: [bool; 8]) -> u8 {
    let mut position: u8 = 8;
    bool_vec[..].into_iter().map(|&p| {
        position -= 1;
        if p {
            1 << position
        }
        else {
            0
        }
    }).fold(0u8, |x,y| {x | y})
}

pub fn plant_position(plant: Plant, biome: Biome) -> u8 {
    match biome {
        Biome::Taiga => match plant {
            _ => 0
        },
        _ => 0,
    }
}
