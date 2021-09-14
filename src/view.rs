mod crafting;

use seed::{prelude::*, *};

use crate::model::{Model, Msg};

pub fn view(model: &Model) -> Vec<Node<Msg>> {
    if let Some(crafting) = model.crafting() {
        crafting::view(crafting)
    } else {
        vec![div!["Loading..."]]
    }
}
