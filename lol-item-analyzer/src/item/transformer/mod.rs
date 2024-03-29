use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::data::transform::Transformer;

use super::Item;

pub mod id_assoc;
pub mod item_value;
pub mod masterwork_assoc;
pub mod masterwork_ident;
pub mod masterwork_item_value;

pub struct ItemTransformContext {
    pub items: Rc<RefCell<HashMap<String, Item>>>,
}

impl ItemTransformContext {
    pub fn new(items: HashMap<String, Item>) -> Self {
        ItemTransformContext {
            items: Rc::new(RefCell::new(items)),
        }
    }
}

#[derive(Clone)]
pub struct TransformHandler {
    pub items: HashMap<String, Item>,
    pub transformers: Vec<Rc<RefCell<dyn Transformer<ItemTransformContext>>>>,
}

impl TransformHandler {
    pub fn new(
        items: HashMap<String, Item>,
        transformers: Vec<Rc<RefCell<dyn Transformer<ItemTransformContext>>>>,
    ) -> Self {
        TransformHandler {
            items,
            transformers,
        }
    }

    /// Transforms all items using all transforms, returning a new map of items. Transforms are
    /// applied one-by-one, in the given order, on all items, before the next transform is applied.
    pub fn transform_all(&self) -> HashMap<String, Item> {
        let mut new_items = HashMap::new();
        new_items.clone_from(&self.items);

        let mut ctx = ItemTransformContext::new(new_items);

        self.transformers
            .iter()
            .for_each(|t| t.borrow_mut().transform(&mut ctx));

        ctx.items.take()
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate;

    use crate::data::transform::MockTransformer;

    use super::*;

    #[test]
    fn transform_all_works() {
        let mut items = HashMap::new();
        items.insert("0000-test".to_string(), Item::default());

        let mock_transformer = Rc::new(RefCell::new(MockTransformer::new()));

        let transform_handler = TransformHandler::new(items, vec![mock_transformer.clone()]);

        mock_transformer
            .borrow_mut()
            .expect_transform()
            .with(predicate::always())
            .once()
            .returning(|_ctx| {});

        transform_handler.transform_all();
    }
}
