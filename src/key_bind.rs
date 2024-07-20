use cosmic::iced::keyboard::Key;
use cosmic::widget::menu::key_bind::KeyBind;
use cosmic::widget::menu::key_bind::Modifier;
use std::collections::HashMap;

use crate::app::Action;

pub fn key_binds() -> HashMap<KeyBind, Action> {
    let mut key_binds = HashMap::new();

    macro_rules! bind {
        ([$($modifier:ident),* $(,)?], $key:expr, $action:ident) => {{
            key_binds.insert(
                KeyBind {
                    modifiers: vec![$(Modifier::$modifier),*],
                    key: $key,
                },
                Action::$action,
            );
        }};
    }

    bind!([Ctrl], Key::Character("c".into()), ChangeCity);
    bind!([Ctrl], Key::Character("q".into()), Quit);
    bind!([Ctrl], Key::Character("i".into()), About);
    bind!([Ctrl], Key::Character(",".into()), Settings);

    key_binds
}
