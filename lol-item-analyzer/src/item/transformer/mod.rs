use std::collections::HashMap;

use super::Item;

pub mod id_assoc;
pub mod masterwork_assoc;
pub mod masterwork_ident;

pub struct TransformContext {
    pub items: HashMap<String, Item>,
}

impl TransformContext {
    pub fn new(items: HashMap<String, Item>) -> Self {
        TransformContext { items }
    }
}

pub struct TransformHandler {
    pub items: HashMap<String, Item>,
    pub init_transformers: Vec<Box<dyn InitTransformer>>,
    pub transformers: Vec<Box<dyn Transformer>>,
}

impl TransformHandler {
    pub fn new(
        items: HashMap<String, Item>,
        init_transformers: Vec<Box<dyn InitTransformer>>,
        transformers: Vec<Box<dyn Transformer>>,
    ) -> Self {
        TransformHandler {
            items,
            init_transformers,
            transformers,
        }
    }

    /// Transforms all items using all transforms, returning a new vector of items. Transforms are
    /// applied one-by-one, in the given order, on all items, before the next transform is applied.
    pub fn transform_all(&mut self) -> HashMap<String, Item> {
        let mut new_items = HashMap::new();
        new_items.clone_from(&self.items);

        let mut ctx = TransformContext::new(new_items);

        self.init_transformers
            .iter()
            .for_each(|t| t.transform(&mut ctx));

        self.transformers.iter().for_each(|t| t.transform(&mut ctx));

        ctx.items
    }
}

/// Perform initial transformation on all items.
pub trait InitTransformer {
    fn transform(&self, ctx: &mut TransformContext);
}

pub trait Transformer {
    /// Transforms a given item into the new item.
    fn transform(&self, ctx: &mut TransformContext);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn transform_all_works() {}
}
