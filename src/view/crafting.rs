use seed::{prelude::*, *};

use crate::model::Msg;

use data::crafting::{CategoryRef, Crafting, ItemRef, RecipeRef};

fn view_item(item: ItemRef) -> Node<Msg> {
    img![C!["item"], attrs![At::Src => item.image_url()]]
}

fn view_ingredient<'a, I: Iterator<Item = ItemRef<'a>>>(items: I) -> Node<Msg> {
    div![C!["ingredient"], items.map(view_item)]
}

fn view_recipe(recipe: RecipeRef) -> Node<Msg> {
    div![
        C!["recipe", "box", "notification", "is-info"],
        div![C!["ingredients"], recipe.input().map(view_ingredient)],
        div![C!["outputs"], recipe.output().map(view_item)]
    ]
}

fn view_main_item(item: ItemRef) -> Node<Msg> {
    div![
        C!["recipe-container", "block"],
        div![C!["box", "notification", "is-info"], view_item(item)],
        div![item.recipes().map(view_recipe)],
        div![item.usage().map(view_recipe)]
    ]
}

fn view_category(category: CategoryRef) -> Node<Msg> {
    section![
        h1![C!["title"], category.name()],
        category.items().map(view_main_item)
    ]
}

pub fn view(crafting: &Crafting) -> Vec<Node<Msg>> {
    crafting.categories().map(view_category).collect()
}
