
use tcod::{colors, Color};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Biome {
    // Water
    Ocean,
    River,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BiomeType {
    Unspecified,
    Water,
    Temperate,
    Dry,
    Wet,
    Tropical,
    Polar,
    Mountain,
}

pub enum BiomeRepresentation {
    Standard(char, Color, Color),
    Ocean,
    Mountain,
    River,
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

    pub fn graphical_representation(self) -> BiomeRepresentation {
        
        match self {
            // Water
            Biome::Ocean => BiomeRepresentation::Ocean,
            Biome::River => BiomeRepresentation::River,
            
            // Polar and Montane
            Biome::Arctic  =>
                BiomeRepresentation::Standard(' ', colors::LIGHTEST_BLUE, colors::LIGHTEST_CYAN),
            Biome::Tundra  =>
                BiomeRepresentation::Standard('~', colors::LIGHTEST_BLUE, colors::LIGHT_CYAN),
            Biome::Taiga   =>
                BiomeRepresentation::Standard('|', colors::DESATURATED_CHARTREUSE, colors::LIGHT_CYAN),
            Biome::Montane =>
                BiomeRepresentation::Standard('^', colors::LIGHTER_GREEN, colors::GREY),

            // Temperate
            Biome::TemperateConiferousForest =>
                BiomeRepresentation::Standard('Y', colors::DARKER_GREEN, colors::DARK_CHARTREUSE),
            Biome::TemperateGrassland =>
                BiomeRepresentation::Standard(',', colors::DARK_CHARTREUSE, colors::CHARTREUSE),
            Biome::TemperateBroadleafForest =>
                BiomeRepresentation::Standard('%', colors::DARK_GREEN, colors::CHARTREUSE),

            // Tropical
            Biome::TropicalConiferousForest =>
                BiomeRepresentation::Standard('Y', colors::LIGHT_GREEN, colors::DESATURATED_GREEN),
            Biome::TropicalDryBroadleafForest =>
                BiomeRepresentation::Standard('%', colors::DESATURATED_GREEN, colors::AMBER),
            Biome::TropicalMoistBroadleafForest =>
                BiomeRepresentation::Standard('%', colors::DESATURATED_CHARTREUSE, colors::DARK_CHARTREUSE),
            Biome::TropicalGrassland =>
                BiomeRepresentation::Standard(',', colors::DARK_LIME, colors::LIGHT_YELLOW),
            
            // Untraversable mountain
            Biome::Mountain => BiomeRepresentation::Mountain,
            
            // Dry
            Biome::Desert         =>
                BiomeRepresentation::Standard('.', colors::DARK_YELLOW, colors::LIGHTER_YELLOW),
            Biome::XericShrubland =>
                BiomeRepresentation::Standard('&', colors::DARK_YELLOW, colors::LIGHT_YELLOW),
            Biome::Woodlands      =>
                BiomeRepresentation::Standard('1', colors::DARKER_AMBER, colors::LIGHT_AMBER),

            // Wet
            Biome::FloodedGrassland =>
                BiomeRepresentation::Standard('~', colors::DARK_GREEN, colors::SEA),
            
            // Default
            _ => BiomeRepresentation::Standard('?', colors::WHITE, colors::BLACK),
        }
    }

    pub fn category(self) -> BiomeType {
        match self {
            Biome::Ocean => BiomeType::Water,
            Biome::River => BiomeType::Water,
            _ => BiomeType::Unspecified,
        }
    }

}
