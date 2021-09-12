mod crafting;

use seed::{prelude::*, *};

use crate::model::{Model, Msg};

pub fn view(model: &Model) -> Node<Msg> {
    if let Some(crafting) = model.crafting() {
        crafting::view(crafting)
    } else {
        div!["Loading..."]
    }
}
