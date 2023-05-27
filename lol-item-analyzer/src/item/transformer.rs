use super::Item;

pub struct TransformContext {
    pub items: Vec<Item>,
}

impl TransformContext {
    pub fn new(items: Vec<Item>) -> Self {
        TransformContext { items }
    }
}

pub struct TransformHandler {
    pub ctx: TransformContext,
    pub transformers: Vec<Box<dyn Transformer>>,
}

impl TransformHandler {
    pub fn new(items: Vec<Item>, transformers: Vec<Box<dyn Transformer>>) -> Self {
        TransformHandler {
            ctx: TransformContext::new(items),
            transformers,
        }
    }

    /// Transforms all items using all transforms, returning a new vector of items. Transforms are
    /// applied one-by-one, in the given order, on all items, before the next transform is applied.
    pub fn transform_all(&self) -> Vec<Item> {
        todo!()
    }
}

pub trait Transformer {
    /// Transforms a given item into the new item.
    fn transform(&self, ctx: TransformContext, item: Item) -> Item;
}
