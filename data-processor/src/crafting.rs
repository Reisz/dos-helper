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

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
enum Ingredient {
    Unique(String),
    Alternatives(Vec<String>),
}

impl Ingredient {
    fn convert(self, item_map: &ItemMap) -> output::Ingredient {
        match self {
            Self::Unique(name) => output::Ingredient::Unique(*item_map.get(&name).expect(&name)),
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
#[serde(untagged)]
enum Recipe {
    Regular {
        input: Vec<Ingredient>,
        output: Vec<String>,
    },
    PotionUpgrade {
        potion_levels: Vec<String>,
        hq_augmentors: Option<u8>,
        ult_augmentors: Option<u8>,
    },
}

const AUGMENTORS: &[&str] = &[
    "Augmentor",
    "High Quality Augmentor",
    "Ultimate Augmentor Herb",
];

impl Recipe {
    fn build_recipe(
        input: Vec<Ingredient>,
        output: Vec<String>,
        item_map: &ItemMap,
    ) -> output::Recipe {
        let mut result = output::Recipe::new();

        for input in input {
            result.add_input(input.convert(item_map));
        }

        for output in output {
            result.add_output(*item_map.get(&output).unwrap())
        }

        result
    }

    fn add_potion_recipes(
        recipes: &mut Vec<output::Recipe>,
        input: String,
        output: String,
        augmentor_count: u8,
        item_map: &ItemMap,
    ) {
        let augmentors = AUGMENTORS
            .iter()
            .cloned()
            .skip(AUGMENTORS.len() - augmentor_count as usize)
            .map(str::to_owned)
            .collect();

        recipes.push(Self::build_recipe(
            vec![Ingredient::Unique(input.clone()); 2],
            vec![output.clone()],
            item_map,
        ));

        recipes.push(Self::build_recipe(
            vec![
                Ingredient::Unique(input.clone()),
                Ingredient::Alternatives(augmentors),
            ],
            vec![output.clone()],
            item_map,
        ));
    }

    fn convert(self, item_map: &ItemMap) -> Vec<output::Recipe> {
        match self {
            Self::Regular { input, output } => {
                vec![Self::build_recipe(input, output, item_map)]
            }
            Self::PotionUpgrade {
                mut potion_levels,
                hq_augmentors,
                ult_augmentors,
            } => {
                let mut recipes = Vec::new();

                let mut hq_augmentors = hq_augmentors.unwrap_or(1);
                let mut ult_augmentors = ult_augmentors.unwrap_or(1);

                potion_levels.reverse();
                for (input, output) in potion_levels.iter().skip(1).zip(potion_levels.iter()) {
                    let (input, output) = (input.to_owned(), output.to_owned());

                    let augmentor_count = if ult_augmentors > 0 {
                        ult_augmentors -= 1;
                        1
                    } else if hq_augmentors > 0 {
                        hq_augmentors -= 1;
                        2
                    } else {
                        3
                    };

                    Self::add_potion_recipes(
                        &mut recipes,
                        input,
                        output,
                        augmentor_count,
                        item_map,
                    );
                }

                recipes
            }
        }
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
        for recipe in recipe.convert(&item_map) {
            crafting.add_recipe(recipe);
        }
    }

    let file = File::create("dist/data.bin.xz").unwrap();
    bincode::serialize_into(XzEncoder::new(file, 9), &crafting).unwrap();
}
