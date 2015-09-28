
use tcod::console::{Root, Console, BackgroundFlag};
use tcod::colors;

use game::Game;
use map::Tile;

pub fn render_screen(game: &Game, root: &mut Root) {
    root.clear();
    root.set_default_foreground(colors::WHITE);
    
    for x in (0..game.map_console.width()) {
        for y in (0..game.map_console.height()) {
            let tile = game.map.get_tile(x as usize + game.camera.x as usize,
                                         y as usize + game.camera.y as usize);
            let (character, fg, bg) = tile.graphical_representation();
            root.put_char_ex(x, y, character, fg, bg);
        }
    }

    
    root.put_char(game.cursor.x - game.camera.x,
                  game.cursor.y - game.camera.y,
                  '@',
                  BackgroundFlag::None);
    
    root.flush();
}

            
