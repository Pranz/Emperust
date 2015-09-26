extern crate tcod;
extern crate yaml_rust;


use tcod::console::{Root, Console, FontLayout, FontType, BackgroundFlag};
use tcod::colors;
use tcod::input::Key;
use tcod::input::KeyCode;

mod settings;

use settings::Settings;

fn main() {
    let settings = Settings::read("settings.yaml").unwrap();
    
    let mut root = Root::initializer()
        .font(&settings.font_path, FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(settings.width, settings.height)
        .title(settings.title)
        .init();

    tcod::system::set_fps(20);

    while !root.window_closed() {
        root.set_default_foreground(colors::WHITE);
        root.put_char(5, 5, '@', BackgroundFlag::None);

        root.flush();
    }
}
