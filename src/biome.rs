
use tcod::{colors, Color};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Biome {
    // Water
    Ocean,
    // Untraversable mountain devoid of much life
    Mountain,
    // Polar and Montane
    Tundra,
    Taiga,
    Arctic,
    Montane,
    // Temperate
    TemperateConiferousForest,
    TemperateBroadleafForest,
    TemperateGrassland, //includes shrublands and shortgrass praire
    // Tropical and subtropical
    TropicalConiferousForest,
    TropicalDryBroadleafForest,
    TropicalMoistBroadleafForest,
    TropicalGrassland, //includes savanna and tropical shrublands
    // Very dry stuff
    Woodlands,
    Desert,
    XericShrubland,
    // Very wet stuff
    FloodedGrassland,
    Wetland,
    Riparian,
}

impl Biome {
    pub fn new(height: u8, temperature: u8, rainfall: u8, tree_line: u8, ocean_line: u8) -> Biome {
        match height {
            x if x < ocean_line => Biome::Ocean,
            x if x >= tree_line => if x > 172 { Biome::Mountain } else { Biome::Montane },
            _ => match (temperature, rainfall) {
                (t, rf) if t > 160 && rf < 105 => Biome::Desert,
                (t, rf) if t > 160 && rf < 122 => Biome::XericShrubland,
                (t, rf) if t > 160 && rf < 135 => Biome::Woodlands,
                (t, _ ) if t < 70              => Biome::Arctic,
                (t, rf) if t < 90  && rf < 130 => Biome::Tundra,
                (t, rf) if t < 90              => Biome::Taiga,
                (t, rf) if t < 141 && rf < 130 => Biome::TemperateGrassland,
                (t, rf) if t < 151 && rf < 150 => Biome::TemperateConiferousForest,
                (t ,rf) if t < 161 && rf < 165 => Biome::TemperateBroadleafForest,
                (t ,rf) if rf > 210            => Biome::Wetland,
                (t, rf) if rf > 164            => Biome::FloodedGrassland,
                (t, rf) if t > 170 && rf > 150 => Biome::TropicalMoistBroadleafForest,
                (t, rf) if t > 170 && rf > 69  => Biome::TropicalDryBroadleafForest,
                (t, rf) if t > 160 && rf > 140 => Biome::TropicalConiferousForest,
                (t, rf) if t > 160 && rf > 129 => Biome::TropicalGrassland,
                _ => Biome::Arctic
            }
        }
    }

    pub fn graphical_representation(self, height: u8) -> (char, Color, Color) {
        match self {
            // Water
            Biome::Ocean => (' ', colors::WHITE, Color::new(0,0, ((height as f32 - 25.0) * 2.0) as u8)),
            // Polar and Montane
            Biome::Arctic  => (' ', colors::LIGHTEST_BLUE, colors::LIGHTEST_CYAN),
            Biome::Tundra  => ('~', colors::LIGHTEST_BLUE, colors::LIGHT_CYAN),
            Biome::Taiga   => ('|', colors::DESATURATED_CHARTREUSE, colors::LIGHT_CYAN),
            Biome::Montane => ('^', colors::LIGHTER_GREEN, colors::GREY),

            // Temperate
            Biome::TemperateConiferousForest =>
                ('Y', colors::DARKER_GREEN, colors::DARK_CHARTREUSE),
            Biome::TemperateGrassland =>
                (',', colors::DARK_CHARTREUSE, colors::CHARTREUSE),
            Biome::TemperateBroadleafForest =>
                ('%', colors::DARK_GREEN, colors::CHARTREUSE),

            // Tropical
            Biome::TropicalConiferousForest =>
                ('Y', colors::LIGHT_GREEN, colors::DESATURATED_GREEN),
            Biome::TropicalDryBroadleafForest =>
                ('%', colors::DESATURATED_GREEN, colors::AMBER),
            Biome::TropicalMoistBroadleafForest =>
                ('%', colors::DESATURATED_CHARTREUSE, colors::DARK_CHARTREUSE),
            Biome::TropicalGrassland =>
                (',', colors::DARK_LIME, colors::LIGHT_YELLOW),
            
            // Untraversable mountain
            Biome::Mountain => {
                let brightness = 50 + (height - 150) * 4;
                let color = Color::new(brightness, brightness, brightness);
                ('^', color, colors::DARK_GREY)
            },

            // Dry
            Biome::Desert => ('.', colors::DARK_YELLOW, colors::LIGHTER_YELLOW),
            Biome::XericShrubland => ('&', colors::DARK_YELLOW, colors::LIGHT_YELLOW),
            Biome::Woodlands => ('1', colors::DARKER_AMBER, colors::LIGHT_AMBER),

            // Wet
            Biome::FloodedGrassland => ('~', colors::DARK_GREEN, colors::SEA),
            
            // Default
            _ => ('?', colors::WHITE, colors::BLACK),
        }
    }

}
