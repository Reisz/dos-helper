use seed::{prelude::*, *};

use crate::model::Msg;

use data::crafting::{Category, Crafting, Item};

fn view_item(item: &Item) -> Vec<Node<Msg>> {
    vec![img![attrs![At::Src => item.image_url()]], div![item.name()]]
}

fn view_main_item(item: &Item) -> Node<Msg> {
    println!("{}", item.name());
    div![
        C!["recipe-container"],
        div![C!["box", "recipe-box"], view_item(item)],
    ]
}

fn view_category(crafting: &Crafting, category: &Category) -> Node<Msg> {
    section![
        h1![C!["title"], category.name()],
        crafting.items(category).iter().map(view_main_item)
    ]
}

pub fn view(crafting: &Crafting) -> Vec<Node<Msg>> {
    crafting
        .categories()
        .iter()
        .map(|category| view_category(crafting, category))
        .collect()
}
