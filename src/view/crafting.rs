use seed::{prelude::*, *};

use crate::model::{
    crafting::{Category, Crafting, Item, Recipe},
    Msg,
};

fn view_item(item: &Item) -> Vec<Node<Msg>> {
    vec![img![attrs![At::Src => item.image_url()]], div![item.name()]]
}

fn view_recipe(crafting: &Crafting, recipe: &Recipe) -> Node<Msg> {
    td![table![tr![crafting
        .recipe_items(recipe)
        .map(|item| td![view_item(item)])]]]
}

fn view_main_item(crafting: &Crafting, item: &Item) -> Node<Msg> {
    println!("{}", item.name());
    tr![
        td![view_item(item)],
        item.recipes()
            .iter()
            .map(|recipe| view_recipe(crafting, recipe))
    ]
}

fn view_category(crafting: &Crafting, category: &Category) -> Vec<Node<Msg>> {
    let mut result = vec![tr![td![h2![category.name()]]]];
    result.extend(
        crafting
            .items(category)
            .iter()
            .map(|item| view_main_item(crafting, item)),
    );
    result
}

pub fn view(crafting: &Crafting) -> Node<Msg> {
    table![crafting
        .categories()
        .iter()
        .map(|category| view_category(crafting, category))]
}
