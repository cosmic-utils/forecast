use std::collections::HashMap;
use cosmic::widget::menu::key_bind::KeyBind;
use cosmic::widget::menu::menu_tree::{menu_items, menu_root, MenuItem};
use cosmic::{
    widget::menu::{ItemHeight, ItemWidth, MenuBar, MenuTree},
    Element,
};

use crate::app::{Action, Message};

pub fn menu_bar<'a>(key_binds: &HashMap<KeyBind, Action>) -> Element<'a, Message> {
    MenuBar::new(vec![
        MenuTree::with_children(
            menu_root("File"),
            menu_items(
                key_binds,
                vec![
                    MenuItem::Button("Quit", Action::Quit),
                ],
            ),
        ),
        MenuTree::with_children(
            menu_root("Edit"),
            menu_items(
                key_binds,
                vec![
                    MenuItem::Button("Add City", Action::AddCity),
                    MenuItem::Button("Remove City", Action::RemoveCity),
                ],
            ),
        ),
        MenuTree::with_children(
            menu_root("View"),
            menu_items(
                key_binds,
                vec![
                    MenuItem::Button("About", Action::About),
                    MenuItem::Button("Settings", Action::Settings),
                ],
            ),
        ),
    ])
    .item_height(ItemHeight::Dynamic(40))
    .item_width(ItemWidth::Uniform(240))
    .spacing(4.0)
    .into()
}
