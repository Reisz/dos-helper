use std::ops::Range;

use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

pub use refs::*;

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
    pub fn new() -> Self {
        Self::default()
    }

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
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Crafting {
    categories: Vec<Category>,
    items: Vec<Item>,
    recipes: Vec<Recipe>,
}

impl Crafting {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn categories(&self) -> impl Iterator<Item = CategoryRef> {
        self.categories
            .iter()
            .map(move |category| CategoryRef::new(category, self))
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

///  Reference wrappers allow for an ergonomic reading API despite index-based storage.
mod refs {
    use super::*;

    #[derive(Debug, Clone, Copy)]
    pub struct RecipeRef<'a> {
        recipe: &'a Recipe,
        crafting: &'a Crafting,
    }

    impl<'a> RecipeRef<'a> {
        fn new(recipe: &'a Recipe, crafting: &'a Crafting) -> Self {
            Self { recipe, crafting }
        }

        pub fn input(&self) -> impl Iterator<Item = impl Iterator<Item = ItemRef>> {
            self.recipe.input.iter().map(move |ingredient| {
                ingredient
                    .into_iter()
                    .map(move |item| ItemRef::new(&self.crafting.items[item], self.crafting))
            })
        }

        pub fn output(&self) -> impl Iterator<Item = ItemRef> {
            self.recipe
                .output
                .iter()
                .cloned()
                .map(move |item| ItemRef::new(&self.crafting.items[item], self.crafting))
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct ItemRef<'a> {
        item: &'a Item,
        crafting: &'a Crafting,
    }

    impl<'a> ItemRef<'a> {
        fn new(item: &'a Item, crafting: &'a Crafting) -> Self {
            Self { item, crafting }
        }

        pub fn name(&self) -> &str {
            &self.item.name
        }

        pub fn image_url(&self) -> &str {
            &self.item.image_url
        }

        pub fn recipes(&self) -> impl Iterator<Item = RecipeRef> {
            self.item
                .recipes
                .iter()
                .cloned()
                .map(move |recipe| RecipeRef::new(&self.crafting.recipes[recipe], self.crafting))
        }

        pub fn usage(&self) -> impl Iterator<Item = RecipeRef> {
            self.item
                .usage
                .iter()
                .cloned()
                .map(move |recipe| RecipeRef::new(&self.crafting.recipes[recipe], self.crafting))
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct CategoryRef<'a> {
        category: &'a Category,
        crafting: &'a Crafting,
    }

    impl<'a> CategoryRef<'a> {
        pub(super) fn new(category: &'a Category, crafting: &'a Crafting) -> Self {
            Self { category, crafting }
        }

        pub fn name(&self) -> &str {
            &self.category.name
        }

        pub fn items(&self) -> impl Iterator<Item = ItemRef> {
            self.crafting.items[self.category.range.clone()]
                .iter()
                .map(move |item| ItemRef::new(item, self.crafting))
        }
    }
}
