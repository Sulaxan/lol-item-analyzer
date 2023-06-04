use crate::item::Item;

use super::{Transformer, TransformContext};

/// Identifies masterwork items.
struct MasterworkIdentifierTransformer;

impl Transformer for MasterworkIdentifierTransformer {
    fn transform(&self, ctx: &TransformContext, item: Item) -> Item {
        item.is_masterwork = item.required_ally == "Ornn";
        item
    }
}
