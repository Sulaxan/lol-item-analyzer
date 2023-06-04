use crate::item::Item;

use super::{Transformer, TransformContext};

struct MasterworkAssociatorTransformer;

impl Transformer for MasterworkAssociatorTransformer {
    fn transform(&self, ctx: &TransformContext, item: Item) -> Item {
        if item.is_masterwork {
            match item.from {
                Some(ids) => item.masterwork_from = ids.get(0).unwrap_or("0"),
                None => _,
            }

            let base_mythic = ctx.items.get(&item.masterwork_from);
            match base_mythic {
                Some(mythic) => mythic.masterwork_into = Some(item.id),
                None => _,
            }
        }

        item
    }
}
