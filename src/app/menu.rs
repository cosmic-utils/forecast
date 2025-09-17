use cosmic::widget::menu::key_bind::KeyBind;
use cosmic::widget::menu::Item;
use cosmic::{
    app::Core,
    widget::{
        menu::{ItemHeight, ItemWidth},
        responsive_menu_bar,
    },
    Element,
};
use std::{collections::HashMap, sync::LazyLock};

use crate::app::{Action, Message};
use crate::fl;

static MENU_ID: LazyLock<cosmic::widget::Id> = LazyLock::new(|| cosmic::widget::Id::new("responsive-menu"));

pub fn menu_bar<'a>(core: &Core, key_binds: &HashMap<KeyBind, Action>) -> Element<'a, Message> {
    responsive_menu_bar()
        .item_height(ItemHeight::Dynamic(40))
        .item_width(ItemWidth::Uniform(240))
        .spacing(4.0)
        .into_element(
            core, 
            key_binds, 
            MENU_ID.clone(), 
            Message::Surface, 
            vec![
                (
                    fl!("file"),
                    vec![
                        Item::Button(fl!("quit"), None, Action::Quit)
                    ]
                ),
                (
                    fl!("edit"),
                    vec![
                        Item::Button(fl!("change-city"), None, Action::ChangeCity),
                        Item::Button(fl!("api-key"), None, Action::ChangeApiKey),
                    ]
                ),
                (
                    fl!("view"),
                    vec![
                        Item::Button(fl!("about"), None, Action::About),
                        Item::Button(fl!("settings"), None, Action::Settings),
                    ]
                )
            ],
        )
}
