#![allow(dead_code)]

extern crate tcod;
extern crate yaml_rust;
extern crate num;
#[macro_use] extern crate itertools;

use tcod::console::{Root, Console, FontLayout, FontType};

mod settings;
mod input;
mod direction;
mod point;
mod game;
mod render;
mod map;
mod biome;

use game::Game;
use settings::Settings;
use input::{handle_input, UserCommand};
use render::render_screen;

fn main() {
    let settings = Settings::read("settings.yaml").unwrap();
    
    let mut root = Root::initializer()
        .font(&settings.font_path, FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(settings.main_window_width, settings.main_window_height)
        .title(&settings.title)
        .init();

    
    let mut game = Game::new(settings);

    tcod::system::set_fps(20);

    while !root.window_closed() {
        render_screen(&mut game, &mut root);
        
        let command = handle_input(&mut root);
        if command == UserCommand::Exit {
            break;
        }
        else {
            game.execute_command(command);
        }
        

    }
}
