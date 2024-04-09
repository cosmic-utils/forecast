use std::collections::HashMap;
use cosmic::widget::menu::key_bind::KeyBind;
use cosmic::widget::menu::menu_tree::{menu_items, menu_root, MenuItem};
use cosmic::{
    widget::menu::{ItemHeight, ItemWidth, MenuBar, MenuTree},
    Element,
};

use crate::app::{Action, Message};
use crate::fl;

pub fn menu_bar<'a>(key_binds: &HashMap<KeyBind, Action>) -> Element<'a, Message> {
    MenuBar::new(vec![
        MenuTree::with_children(
            menu_root(fl!("file")),
            menu_items(
                key_binds,
                vec![
                    MenuItem::Button(fl!("quit"), Action::Quit),
                ],
            ),
        ),
        MenuTree::with_children(
            menu_root(fl!("edit")),
            menu_items(
                key_binds,
                vec![
                    MenuItem::Button(fl!("change-city"), Action::ChangeCity),
                ],
            ),
        ),
        MenuTree::with_children(
            menu_root(fl!("view")),
            menu_items(
                key_binds,
                vec![
                    MenuItem::Button(fl!("about"), Action::About),
                    MenuItem::Button(fl!("settings"), Action::Settings),
                ],
            ),
        ),
    ])
    .item_height(ItemHeight::Dynamic(40))
    .item_width(ItemWidth::Uniform(240))
    .spacing(4.0)
    .into()
}
