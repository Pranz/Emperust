#![allow(dead_code)]

extern crate tcod;
extern crate yaml_rust;
extern crate num;
#[macro_use] extern crate itertools;

use tcod::console::{Root, Console, FontLayout, FontType};

use std::thread;
use std::sync::mpsc::channel;

mod settings;
mod input;
mod direction;
mod point;
mod game;
mod render;
mod map;
mod biome;
mod world_gen;

use game::Game;
use settings::Settings;
use input::{handle_input, UserCommand};
use render::{render_screen, render_progress};

fn main() {
    let settings = Settings::read("settings.yaml").unwrap();
    
    let mut root = Root::initializer()
        .font(&settings.font_path, FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(settings.main_window_width, settings.main_window_height)
        .title(&settings.title)
        .init();

    tcod::system::set_fps(20);

    let (tx, rx) = channel();
    let width = settings.map_width;
    let t = thread::spawn(move || {
        render_progress(&mut root, width , rx);
        return root;
    });
    
    let mut game = Game::new(settings, tx);
    let mut root = t.join().unwrap();

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
