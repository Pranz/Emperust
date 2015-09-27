
type Tile = u8;

pub struct Map {
    tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Map {
        let mut tiles: Vec<Tile> = Vec::new();
        for _ in (0..(width * height)) {
            tiles.push(0);
        }
        Map {
            tiles: tiles,
            width: width,
            height: height,
        }
    }
}
