use super::{Transformer, TransformContext};

/// Identifies masterwork items.
struct MasterworkIdentifierTransformer;

impl Transformer for MasterworkIdentifierTransformer {
    fn transform(&self, ctx: &mut TransformContext) {
        ctx.items.iter_mut().for_each(|(id, item)| {
            let is_masterwork = item.required_ally.is_some() && item.required_ally.as_ref().unwrap() == "Ornn";
            item.is_masterwork = is_masterwork;
        });
    }
}
