use seed::prelude::*;

use data::crafting::Crafting;
use lzma_rs::xz_decompress;

#[derive(Debug, Default)]
pub struct Model {
    crafting: Option<Crafting>,
}

impl Model {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn crafting(&self) -> Option<&Crafting> {
        self.crafting.as_ref()
    }
}

pub enum Msg {
    DataLoaded(Crafting),
}

pub fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.perform_cmd(load_data());
    Model::new()
}

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::DataLoaded(crafting) => {
            debug_assert!(model.crafting.is_none());
            model.crafting = Some(crafting);
        }
    }
}

async fn load_data() -> Msg {
    let request = Request::new("/data.bin.xz").cache(web_sys::RequestCache::NoCache);
    let response = request.fetch().await.unwrap();
    let bytes = response.bytes().await.unwrap();

    let mut decompressed = Vec::new();
    xz_decompress(&mut bytes.as_slice(), &mut decompressed).unwrap();
    Msg::DataLoaded(bincode::deserialize(&decompressed).unwrap())
}
