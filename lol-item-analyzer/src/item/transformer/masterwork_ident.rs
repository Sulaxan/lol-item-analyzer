use crate::item::Item;

use super::{Transformer, TransformContext};

/// Identifies masterwork items.
struct MasterworkIdentifierTransformer;

impl Transformer for MasterworkIdentifierTransformer {
    fn transform(&self, _ctx: &mut TransformContext, item: &mut Item) {
        let is_masterwork = item.required_ally.is_some() && item.required_ally.as_ref().unwrap() == "Ornn";
        item.is_masterwork = is_masterwork;
    }
}
