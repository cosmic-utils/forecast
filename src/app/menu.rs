use cosmic::widget::menu::key_bind::KeyBind;
use cosmic::widget::menu::{items, root, Item};
use cosmic::{
    widget::menu::{ItemHeight, ItemWidth, MenuBar, Tree},
    Element,
};
use std::collections::HashMap;

use crate::app::{Action, Message};
use crate::fl;

pub fn menu_bar<'a>(key_binds: &HashMap<KeyBind, Action>) -> Element<'a, Message> {
    MenuBar::new(vec![
        Tree::with_children(
            root(fl!("file")),
            items(key_binds, vec![Item::Button(fl!("quit"), Action::Quit)]),
        ),
        Tree::with_children(
            root(fl!("edit")),
            items(
                key_binds,
                vec![
                    Item::Button(fl!("change-city"), Action::ChangeCity),
                    Item::Button(fl!("api-key"), Action::ChangeApiKey),
                ],
            ),
        ),
        Tree::with_children(
            root(fl!("view")),
            items(
                key_binds,
                vec![
                    Item::Button(fl!("about"), Action::About),
                    Item::Button(fl!("settings"), Action::Settings),
                ],
            ),
        ),
    ])
    .item_height(ItemHeight::Dynamic(40))
    .item_width(ItemWidth::Uniform(240))
    .spacing(4.0)
    .into()
}
