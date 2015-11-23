
use tcod::console::{Root, Console, BackgroundFlag, blit};
use tcod::colors;

use itertools::Product;
use std::sync::mpsc::Receiver;

use biome::{Biome, BiomeRepresentation};
use game::{Game, ProgressInfo};
use map::Tile;

pub fn render_screen(game: &mut Game, root: &mut Root) {
    root.clear();
    root.set_default_foreground(colors::WHITE);

    render_map_zoomed_in(game);
    render_map_zoomed_out(game);
    render_debug_info(game);
    
    blit(&game.map_console, (0,0), (0,0), root, (0,0), 1.0, 1.0);
    blit(&game.zoomed_map_console, (0,0), (0,0), root, (game.map_console.width() + 1, 0), 1.0, 1.0);
    blit(&game.debug_console, (0,0), (0,0), root, (0, game.map_console.height() + 1), 1.0, 1.0);
    root.flush();
}

pub fn render_map_zoomed_in(game: &mut Game) {
    let (con, cursor, camera, map) = (&mut game.map_console, game.cursor, game.camera, &game.map);

    for x in (0..con.width()) {
        for y in (0..con.height()) {
            let tile = map.get_tile(x as usize + game.camera.x as usize,
                                    y as usize + game.camera.y as usize);
            let (character, fg, bg) = tile.graphical_representation(map);
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
        let biome = map.get_biome(x as usize, y as usize);
        let (character, fg ,bg) = match biome.graphical_representation() {
            BiomeRepresentation::Ocean => (' ', colors::WHITE, colors::DARK_BLUE),
            BiomeRepresentation::Mountain => ('^', colors::LIGHT_GREY, colors::DARK_GREY),
            BiomeRepresentation::Standard(chr, fg, bg) => (chr, fg, bg),
            BiomeRepresentation::River => (' ', colors::WHITE, colors::DARK_BLUE),
        };
        con.put_char_ex(x, y, character, fg, bg);
    }

    con.put_char(cursor.x, cursor.y, '@', BackgroundFlag::None);
}

pub fn render_debug_info(game: &mut Game) {
    let (cursor, con, map) = (game.cursor, &mut game.debug_console, &game.map);
    con.clear();

    let tile = map.get_tile(cursor.x as usize, cursor.y as usize);
    let info: [String; 6] = ["Position x: ".to_string() + &cursor.x.to_string(),
                             "Position y: ".to_string() + &cursor.y.to_string(),
                             "Height: ".to_string() + &tile.height.to_string(),
                             "Temperature: ".to_string() + &tile.temperature.to_string(),
                             "Rainfall: ".to_string() + &tile.rainfall.to_string(),
                             format!("Biome: {:?}", &tile.biome).to_string()];

    for (i, text) in info.iter().enumerate() {
        con.print(0, i as i32, text);
    }

}

pub fn render_progress(root: &mut Root, width: usize, rx: Receiver<ProgressInfo>){
    root.set_default_foreground(colors::WHITE);
    root.print(3, 3, "Generating map: ");
    print!("yo\n");
    loop {
        let info = {
            let mut info = ProgressInfo::Done;
            loop {
                match rx.try_recv() {
                    Ok(x) => { info = x; },
                    Err(_) => { break; },
                }
            }
            info
        };
        root.flush();
        root.clear();
        match info {
            ProgressInfo::FinishedColumn(n) => {
                root.print(19, 3,n.to_string() + " / " + &width.to_string());
            },
            ProgressInfo::Done => { return (); },
        }
    }
}
