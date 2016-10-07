#![allow(dead_code)]
//#![feature(iter_cmp)]

extern crate emperust;
extern crate tcod;
extern crate yaml_rust;
extern crate num;
extern crate rand;
#[macro_use] extern crate itertools;

use tcod::console::{Root, FontLayout, FontType};

use std::thread;
use std::sync::mpsc::channel;

use emperust::game::Game;
use emperust::settings::Settings;
use emperust::input::{handle_input, UserCommand};
use emperust::render::{render_screen, render_progress};

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

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    while !root.window_closed() {
        render_screen(&mut game, &mut root);
        
        let command = handle_input(&mut root);
        if command == UserCommand::Exit {
            return;
        }
        if command == UserCommand::ChooseSite {
            x = game.cursor.x;
            y = game.cursor.y;
            break;
        }
        
        else {
            game.execute_command(command);
        }
    }
    println!("{} {}", x, y);
}
