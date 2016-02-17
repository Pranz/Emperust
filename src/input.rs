
use tcod::console::Root;
use tcod::input::{Key, KeyCode};

use direction::Direction;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum UserCommand {
    Noop,
    Exit,
    Move(Direction, bool),
    RegenMap,
    CreateRiver,
}

pub fn handle_input(root: &mut Root) -> UserCommand {
    let keypress = root.wait_for_keypress(true);
    if keypress.pressed {
        match keypress {
            Key {code: KeyCode::Escape, ..} => UserCommand::Exit,
            Key {code: KeyCode::Left  , ..} => UserCommand::Move(Direction::Left, keypress.shift),
            Key {code: KeyCode::Right , ..} => UserCommand::Move(Direction::Right, keypress.shift),
            Key {code: KeyCode::Up    , ..} => UserCommand::Move(Direction::Up, keypress.shift),
            Key {code: KeyCode::Down  , ..} => UserCommand::Move(Direction::Down, keypress.shift),
            Key {printable: 'r'       , ..} => UserCommand::RegenMap,
            Key {printable: 'c'       , ..} => UserCommand::CreateRiver,
            _ => handle_input(root),
        }
    }
    else {
        UserCommand::Noop
    }
}
