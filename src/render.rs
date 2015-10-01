
use tcod::console::{Root, Console, BackgroundFlag, blit};
use tcod::colors;

use itertools::Product;

use game::Game;
use map::Tile;

pub fn render_screen(game: &mut Game, root: &mut Root) {
    root.clear();
    root.set_default_foreground(colors::WHITE);

    render_map_zoomed_in(game);
    render_map_zoomed_out(game);
    
    blit(&game.map_console, (0,0), (0,0), root, (0,0), 1.0, 1.0);
    blit(&game.zoomed_map_console, (0,0), (0,0), root, (game.map_console.width() + 1, 0), 1.0, 1.0);
    root.flush();
}

pub fn render_map_zoomed_in(game: &mut Game) {
    let (con, cursor, camera, map) = (&mut game.map_console, game.cursor, game.camera, &game.map);
    

    for x in (0..con.width()) {
        for y in (0..con.height()) {
            let tile = map.get_tile(x as usize + game.camera.x as usize,
                                    y as usize + game.camera.y as usize);
            let (character, fg, bg) = tile.graphical_representation();
            con.put_char_ex(x, y, character, fg, bg);
        }
    }

    
    con.put_char(cursor.x - camera.x,
                 cursor.y - camera.y,
                 '@',
                 BackgroundFlag::None);
}

pub fn render_map_zoomed_out(game: &mut Game) {
    let (cursor, con, map) = (game.get_zoomed_out_cursor(), &mut game.zoomed_map_console, &game.zoomed_map);

    for (x,y) in Product::new((0..con.width()), (0..con.height())) {
        let tile = map.get_tile(x as usize, y as usize);
        let (character, fg ,bg) = tile.graphical_representation();
        con.put_char_ex(x, y, character, fg, bg);
    }

    con.put_char(cursor.x, cursor.y, '@', BackgroundFlag::None);

}
