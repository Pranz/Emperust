
use tcod::console::Root;
use tcod::input::{Key, KeyCode};

use direction::Direction;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum UserCommand {
    Exit,
    Move(Direction),
}

pub fn handle_input(root: &mut Root) -> UserCommand {
    let keypress = root.wait_for_keypress(true);

    match keypress.key {
        Key::Special(KeyCode::Escape) => UserCommand::Exit,
        Key::Special(KeyCode::Left)   => UserCommand::Move(Direction::Left),
        Key::Special(KeyCode::Right)  => UserCommand::Move(Direction::Right),
        Key::Special(KeyCode::Up)     => UserCommand::Move(Direction::Up),
        Key::Special(KeyCode::Down)   => UserCommand::Move(Direction::Down),
        _ => handle_input(root),
    }
}
