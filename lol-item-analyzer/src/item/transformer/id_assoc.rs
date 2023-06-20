use crate::data::transform::Transformer;

use super::ItemTransformContext;

/// Gives all items an ID based on the ID of the key for each value in the map.
pub struct IdAssociatorTransformer;

impl Transformer<ItemTransformContext> for IdAssociatorTransformer {
    fn transform(&self, ctx: &mut ItemTransformContext) {
        ctx.items
            .borrow_mut()
            .iter_mut()
            .for_each(|(id, item)| item.id = id.to_owned());
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::item::Item;

    use super::*;

    #[test]
    fn transform_works() {
        let mock_item_id = "some_id";
        let mock_item_id_2 = "some_id_2";
        let mock_items = HashMap::from([
            (mock_item_id.to_string(), Item::default()),
            (mock_item_id_2.to_string(), Item::default()),
        ]);
        let mut mock_context = ItemTransformContext::new(mock_items);

        let transformer = IdAssociatorTransformer;
        transformer.transform(&mut mock_context);

        assert_eq!(
            mock_context.items.borrow().get(mock_item_id).unwrap().id,
            mock_item_id
        );
        assert_eq!(
            mock_context.items.borrow().get(mock_item_id_2).unwrap().id,
            mock_item_id_2
        );
    }
}
