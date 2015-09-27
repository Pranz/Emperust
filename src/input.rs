
use tcod::console::Root;
use tcod::input::{Key, KeyCode};

use direction::Direction;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum UserCommand {
    Exit,
    Move(Direction),
    RegenMap,
}

pub fn handle_input(root: &mut Root) -> UserCommand {
    let keypress = root.wait_for_keypress(true);

    match keypress {
        Key {code: KeyCode::Escape, ..} => UserCommand::Exit,
        Key {code: KeyCode::Left  , ..} => UserCommand::Move(Direction::Left),
        Key {code: KeyCode::Right , ..} => UserCommand::Move(Direction::Right),
        Key {code: KeyCode::Up    , ..} => UserCommand::Move(Direction::Up),
        Key {code: KeyCode::Down  , ..} => UserCommand::Move(Direction::Down),
        Key {printable: 'r'       , ..} => UserCommand::RegenMap,
        _ => handle_input(root),
    }
}
