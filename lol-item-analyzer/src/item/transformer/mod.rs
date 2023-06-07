use std::{cell::RefCell, collections::HashMap};

use super::Item;

pub mod id_assoc;
pub mod masterwork_assoc;
pub mod masterwork_ident;

pub struct TransformContext {
    pub items: HashMap<String, Item>,
}

impl TransformContext {
    pub fn new(items: HashMap<String, Item>) -> Self {
        // let mut new_items = HashMap::new();
        // items.drain().for_each(|(id, item)| {
        //     new_items.insert(id, RefCell::new(item)));
        // });
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

        &self
            .init_transformers
            .iter()
            .for_each(|t| t.transform(&mut ctx));

        //TODO: figure out a way to clone the keys and release the borrow
        let item_keys = ctx.items.keys().clone();

        &self.transformers.iter().for_each(|t| {
            item_keys.for_each(|id| {
                t.transform(&mut ctx, ctx.items.get_mut(id.as_str()).unwrap()); // should be safe unwrap
            });
        });

        ctx.items
    }
}

/// Perform initial transformation on all items.
pub trait InitTransformer {
    fn transform(&self, ctx: &mut TransformContext);
}

pub trait Transformer {
    /// Transforms a given item into the new item.
    fn transform(&self, ctx: &mut TransformContext, item: &mut Item);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn transform_all_works() {}
}
