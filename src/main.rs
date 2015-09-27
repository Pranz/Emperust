#![allow(dead_code)]

extern crate tcod;
extern crate yaml_rust;

use tcod::console::{Root, Console, FontLayout, FontType};

mod settings;
mod input;
mod direction;
mod point;
mod game;
mod render;
mod map;

use game::Game;
use settings::Settings;
use input::{handle_input, UserCommand};
use render::render_screen;

fn main() {
    let settings = Settings::read("settings.yaml").unwrap();
    let mut game = Game::new(&settings);
    
    let mut root = Root::initializer()
        .font(&settings.font_path, FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(settings.width, settings.height)
        .title(settings.title)
        .init();

    tcod::system::set_fps(20);

    while !root.window_closed() {
        render_screen(&game, &mut root);
        
        let command = handle_input(&mut root);
        if command == UserCommand::Exit {
            break;
        }
        else {
            game.execute_command(command);
        }
        

    }
}
