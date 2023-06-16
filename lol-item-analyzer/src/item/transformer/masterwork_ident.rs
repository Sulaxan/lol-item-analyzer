use super::{TransformContext, Transformer};

/// Identifies masterwork items.
struct MasterworkIdentifierTransformer;

impl Transformer for MasterworkIdentifierTransformer {
    fn transform(&self, ctx: &mut TransformContext) {
        ctx.items.borrow_mut().iter_mut().for_each(|(_id, item)| {
            let is_masterwork =
                item.required_ally.is_some() && item.required_ally.as_ref().unwrap() == "Ornn";
            item.is_masterwork = is_masterwork;
        });
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::item::Item;

    use super::*;

    #[test]
    fn transform_works() {
        let test_item_id = "test";
        let transformer = MasterworkIdentifierTransformer;
        let mock_items = HashMap::from([(test_item_id.to_string(), {
            let mut item = Item::default();
            item.required_ally = Some("Ornn".to_string());
            item
        })]);
        let mut mock_context = TransformContext::new(mock_items);

        transformer.transform(&mut mock_context);

        assert_eq!(
            mock_context
                .items
                .borrow()
                .get(test_item_id)
                .unwrap()
                .is_masterwork,
            true
        );
    }
}
