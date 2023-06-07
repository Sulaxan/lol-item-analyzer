use crate::item::Item;

use super::{TransformContext, Transformer};

struct MasterworkAssociatorTransformer;

impl Transformer for MasterworkAssociatorTransformer {
    fn transform(&self, ctx: &mut TransformContext, masterwork_item: &mut Item) {
        if masterwork_item.is_masterwork {
            match masterwork_item.from.as_ref() {
                Some(ids) => {
                    // the base mythic item id is contained as the first element in "ids" (from API)
                    masterwork_item.masterwork_from = ids.get(0).unwrap_or(&"0".to_string()).to_string()
                }
                None => {}
            }

            let masterwork_base_mythic = ctx.items.get_mut(&masterwork_item.masterwork_from);
            match masterwork_base_mythic {
                Some(mythic) => mythic.masterwork_into = Some(masterwork_item.id.to_owned()),
                None => {}
            }
        }
    }
}
