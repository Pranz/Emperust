
use tcod::{colors, Color};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Biome {
    Ocean,
    Plains,
    Mountain,
    Desert,
    BorealForest,
    Arctic,
}

impl Biome {
    pub fn new(height: u8, temperature: u8, tree_line: u8, ocean_line: u8) -> Biome {
        if height < ocean_line
        { Biome::Ocean } else if height >= tree_line
        { Biome::Mountain } else {
            if temperature > 170
            { Biome::Desert } else if temperature > 130
            { Biome::Plains } else if temperature > 90
            { Biome::BorealForest} else
            { Biome::Arctic}
        }
    }

    pub fn graphical_representation(self, height: u8) -> (char, Color, Color) {
        match self {
            Biome::Ocean => (' ', colors::WHITE, Color::new(0,0, ((height as f32 - 25.0) * 2.0) as u8)),
            Biome::Plains => (',', colors::DARKER_CHARTREUSE, colors::DARK_CHARTREUSE),
            Biome::Mountain => ('^', colors::GREY, colors::DARK_GREY),
            Biome::Desert => ('.', colors::LIGHTER_YELLOW, colors::LIGHT_YELLOW),
            Biome::BorealForest => ('}', colors::DARK_GREEN, colors::DARKER_CHARTREUSE),
            Biome::Arctic => ('-', colors::LIGHTEST_BLUE, colors::LIGHT_TURQUOISE),
        }
    }

}
