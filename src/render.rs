
use tcod::console::{Root, Console, BackgroundFlag};
use tcod::colors;

use game::Game;

pub fn render_screen(game: &Game, root: &mut Root) {
    root.clear();
    root.set_default_foreground(colors::WHITE);
    root.put_char(game.cursor.x, game.cursor.y, '@', BackgroundFlag::None);
    root.flush();
}
