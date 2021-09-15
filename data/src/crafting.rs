use std::ops::Range;

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

#[derive(Debug, Deserialize, Serialize)]
pub enum Ingredient {
    Unique(usize),
    Alternatives(SmallVec<[usize; 2]>),
}

impl<'a> IntoIterator for &'a Ingredient {
    type Item = usize;
    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Ingredient::Unique(idx) => Box::new(std::iter::once(*idx)),
            Ingredient::Alternatives(alts) => Box::new(alts.iter().cloned()),
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Recipe {
    input: SmallVec<[Ingredient; 4]>,
    output: SmallVec<[usize; 2]>,
}

impl Recipe {
    pub fn add_input(&mut self, input: Ingredient) {
        self.input.push(input);
    }

    pub fn add_output(&mut self, output: usize) {
        self.output.push(output);
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    name: String,
    image_url: String,
    recipes: SmallVec<[usize; 2]>,
    usage: SmallVec<[usize; 1]>,
}

impl Item {
    pub fn new(name: String, image_url: String) -> Self {
        Self {
            name,
            image_url,
            recipes: SmallVec::new(),
            usage: SmallVec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn image_url(&self) -> &str {
        &self.image_url
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Category {
    name: String,
    range: Range<usize>,
}

impl Category {
    pub fn new(name: String, start: usize) -> Self {
        Self {
            name,
            range: start..start,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Crafting {
    categories: Vec<Category>,
    items: Vec<Item>,
    recipes: Vec<Recipe>,
}

impl Crafting {
    pub fn categories(&self) -> &[Category] {
        &self.categories
    }

    pub fn items(&self, category: &Category) -> &[Item] {
        &self.items[category.range.clone()]
    }

    pub fn add_category(&mut self, name: String) {
        self.categories.push(Category::new(name, self.items.len()));
    }

    /// Adds an item to the last category created and return the index.
    pub fn add_item(&mut self, item: Item) -> usize {
        let pos = self.items.len();
        self.categories.last_mut().unwrap().range.end += 1;
        self.items.push(item);
        pos
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        let pos = self.recipes.len();

        for input in recipe.input.iter().flatten() {
            self.items[input].usage.push(pos)
        }

        for output in recipe.output.iter().cloned() {
            self.items[output].recipes.push(output);
        }

        self.recipes.push(recipe);
    }
}
