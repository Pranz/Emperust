
use tcod::console::{Root, Console, BackgroundFlag};
use tcod::colors;

use game::Game;

pub fn render_screen(game: &Game, root: &mut Root) {
    root.clear();
    root.set_default_foreground(colors::WHITE);
    
    for x in (0..game.map_console.width()) {
        for y in (0..game.map_console.height()) {
            let height = game.map.get_tile(x as usize + game.camera.x as usize,
                                           y as usize + game.camera.y as usize) as u8;
            root.put_char_ex(x,
                             y,
                             ' ',
                             colors::WHITE,
                             colors::Color::new(0, 0, height));
        }
    }

    
    root.put_char(game.cursor.x - game.camera.x,
                  game.cursor.y - game.camera.y,
                  '@',
                  BackgroundFlag::None);
    
    root.flush();
}

            
