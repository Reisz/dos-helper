mod model;
mod view;

use seed::prelude::*;

fn main() {
    App::start("app", model::init, model::update, view::view);
}
