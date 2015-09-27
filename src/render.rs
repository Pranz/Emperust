
use tcod::console::{Root, Console, BackgroundFlag};
use tcod::colors;

use game::Game;

pub fn render_screen(game: &Game, root: &mut Root) {
    root.clear();
    root.set_default_foreground(colors::WHITE);
    
    for x in (0..50) {
        for y in (0..50) {
            let height = game.map.get_tile(x, y) as u8;
            root.put_char_ex(x as i32,
                             y as i32,
                             ' ',
                             colors::WHITE,
                             colors::Color::new(0, 0, height));
        }
    }

    
    root.put_char(game.cursor.x, game.cursor.y, '@', BackgroundFlag::None);
    
    root.flush();
}

            
