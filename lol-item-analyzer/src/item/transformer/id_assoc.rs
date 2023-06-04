use std::collections::HashMap;

use crate::item::Item;

use super::{InitTransformer, TransformContext};

/// Gives all items an ID based on the ID of the key for each value in the map.
struct IdAssociatorTransformer;

impl InitTransformer for IdAssociatorTransformer {
    fn transform(&self, ctx: &mut TransformContext) {
        ctx.items.iter_mut().for_each(|(id, item)| item.id = id);
    }
}
