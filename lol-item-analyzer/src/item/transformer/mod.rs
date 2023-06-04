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
        //     new_items.insert(id, RefCell::new(item));
        // });
        TransformContext { items }
    }
}

pub struct TransformHandler {
    pub ctx: TransformContext,
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
            ctx: TransformContext::new(items),
            init_transformers,
            transformers,
        }
    }

    /// Transforms all items using all transforms, returning a new vector of items. Transforms are
    /// applied one-by-one, in the given order, on all items, before the next transform is applied.
    pub fn transform_all(&mut self) -> HashMap<String, Item> {
        &self
            .init_transformers
            .iter()
            .for_each(|t| t.transform(&mut self.ctx));
        &self.transformers.iter().for_each(|t| {
            &self.ctx.items.iter_mut().for_each(|(id, item)| {
                // this might be illegal since we're giving another mutable reference? -- might
                // need to change to RefCell (or maybe it might not work unless we change the
                // signature of the transform method to be transform(..., RefCell<Item>))
                *item = t.transform(&self.ctx, *item);
            });
        });

        todo!();
    }
}

/// Perform initial transformation on all items.
pub trait InitTransformer {
    fn transform(&self, ctx: &mut TransformContext);
}

pub trait Transformer {
    /// Transforms a given item into the new item.
    fn transform(&self, ctx: &TransformContext, item: Item) -> Item;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn transform_all_works() {}
}
