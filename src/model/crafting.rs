use std::ops::Range;

use serde::Deserialize;
use smallvec::SmallVec;

#[derive(Debug, Deserialize)]
pub struct Recipe {
    ingredients: SmallVec<[usize; 4]>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    name: String,
    image_url: String,
    #[serde(default)]
    recipes: Vec<Recipe>,
}

impl Item {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn image_url(&self) -> &str {
        &self.image_url
    }

    pub fn recipes(&self) -> &[Recipe] {
        &self.recipes
    }
}

#[derive(Debug, Deserialize)]
pub struct Category {
    name: String,
    range: Range<usize>,
}

impl Category {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Deserialize)]
pub struct Crafting {
    items: Vec<Item>,
    categories: Vec<Category>,
}

impl Crafting {
    pub fn categories(&self) -> &[Category] {
        &self.categories
    }

    pub fn items(&self, category: &Category) -> &[Item] {
        &self.items[category.range.clone()]
    }

    pub fn recipe_items<'a>(&'a self, recipe: &'a Recipe) -> impl Iterator<Item = &Item> + 'a {
        recipe.ingredients.iter().map(move |idx| &self.items[*idx])
    }
}
