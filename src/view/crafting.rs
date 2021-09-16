use seed::{prelude::*, *};

use crate::model::Msg;

use data::crafting::{CategoryRef, Crafting, ItemRef};

fn view_item(item: ItemRef) -> Node<Msg> {
    img![attrs![At::Src => item.image_url()]]
}

fn view_main_item(item: ItemRef) -> Node<Msg> {
    println!("{}", item.name());
    div![
        C!["recipe-container"],
        div![C!["box", "recipe-box"], view_item(item)],
    ]
}

fn view_category(category: CategoryRef) -> Node<Msg> {
    section![
        h1![C!["title"], category.name()],
        category.items().map(view_main_item)
    ]
}

pub fn view(crafting: &Crafting) -> Vec<Node<Msg>> {
    crafting
        .categories()
        .map(|category| view_category(category))
        .collect()
}
