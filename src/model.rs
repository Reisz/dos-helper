pub mod crafting;

use seed::prelude::*;

use crafting::Crafting;

#[derive(Debug, Default)]
pub struct Model {
    crafting: Option<Crafting>,
}

impl Model {
    pub fn crafting(&self) -> Option<&Crafting> {
        self.crafting.as_ref()
    }
}

pub enum Msg {
    DataLoaded(Crafting),
}

pub fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(async { Msg::DataLoaded(load_data().await) });
    Model::default()
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::DataLoaded(crafting) => {
            debug_assert!(model.crafting.is_none());
            model.crafting = Some(crafting);
        }
    }
}

async fn load_data() -> Crafting {
    let request = Request::new("data.json").cache(web_sys::RequestCache::NoCache);
    let response = request.fetch().await.unwrap();
    response.json().await.unwrap()
}
