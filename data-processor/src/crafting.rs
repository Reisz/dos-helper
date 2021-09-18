use std::{collections::HashMap, fs::File, io::BufReader};

use data::crafting as output;
use serde::{Deserialize, Serialize};
use xz2::write::XzEncoder;

type Items = Vec<Category>;
type ItemMap = HashMap<String, usize>;

#[derive(Debug, Deserialize, Serialize)]
struct Category {
    name: String,
    items: Vec<Item>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Item {
    name: String,
    image_url: String,
}

impl Item {
    fn convert(self) -> output::Item {
        output::Item::new(self.name, self.image_url)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum Ingredient {
    Unique(String),
    Alternatives(Vec<String>),
}

impl Ingredient {
    fn convert(self, item_map: &ItemMap) -> output::Ingredient {
        match self {
            Self::Unique(name) => output::Ingredient::Unique(*item_map.get(&name).unwrap()),
            Self::Alternatives(alts) => output::Ingredient::Alternatives(
                alts.iter()
                    .map(|name| *item_map.get(name).unwrap())
                    .collect(),
            ),
        }
    }
}

type Recipes = Vec<Recipe>;

#[derive(Debug, Deserialize, Serialize)]
struct Recipe {
    input: Vec<Ingredient>,
    output: Vec<String>,
}

impl Recipe {
    fn convert(self, item_map: &ItemMap) -> output::Recipe {
        let mut result = output::Recipe::new();

        for input in self.input {
            result.add_input(input.convert(item_map));
        }

        for output in self.output {
            result.add_output(*item_map.get(&output).unwrap())
        }

        result
    }
}

pub fn process() {
    let items: Items = {
        let file = File::open("resources/items.yml").unwrap();
        serde_yaml::from_reader(BufReader::new(file)).unwrap()
    };

    let recipes: Recipes = {
        let file = File::open("resources/recipes.yml").unwrap();
        serde_yaml::from_reader(BufReader::new(file)).unwrap()
    };

    let mut crafting = output::Crafting::new();
    let mut item_map: HashMap<String, usize> = HashMap::new();

    for category in items {
        crafting.add_category(category.name);
        for item in category.items {
            let name = item.name.to_owned();
            let item = item.convert();
            item_map.insert(name, crafting.add_item(item));
        }
    }

    for recipe in recipes {
        crafting.add_recipe(recipe.convert(&item_map))
    }

    let file = File::create("dist/data.bin.xz").unwrap();
    bincode::serialize_into(XzEncoder::new(file, 9), &crafting).unwrap();
}
