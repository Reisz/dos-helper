use seed::{prelude::*, *};

use crate::model::{
    crafting::{Category, Crafting, Item, Recipe},
    Msg,
};

fn view_item(item: &Item) -> Vec<Node<Msg>> {
    vec![img![attrs![At::Src => item.image_url()]], div![item.name()]]
}

fn view_recipe(crafting: &Crafting, recipe: &Recipe) -> Node<Msg> {
    div![
        C!["box", "recipe-box"],
        crafting.recipe_items(recipe).map(view_item)
    ]
}

fn view_main_item(crafting: &Crafting, item: &Item) -> Node<Msg> {
    println!("{}", item.name());
    div![
        C!["recipe-container"],
        div![C!["box", "recipe-box"], view_item(item)],
        item.recipes()
            .iter()
            .map(|recipe| view_recipe(crafting, recipe))
    ]
}

fn view_category(crafting: &Crafting, category: &Category) -> Node<Msg> {
    section![
        h1![C!["title"], category.name()],
        crafting
            .items(category)
            .iter()
            .map(|item| view_main_item(crafting, item))
    ]
}

pub fn view(crafting: &Crafting) -> Vec<Node<Msg>> {
    crafting
        .categories()
        .iter()
        .map(|category| view_category(crafting, category))
        .collect()
}
