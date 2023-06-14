use super::{TransformContext, Transformer};

struct MasterworkAssociatorTransformer;

impl Transformer for MasterworkAssociatorTransformer {
    fn transform(&self, ctx: &mut TransformContext) {
        // (mythic_id, masterwork_id)
        // this vector exists for the sole purpose of mapping the regular mythic items to
        // masterwork items. we, unfortunately, can't do this at the same time as the original
        // iteration through all the items since we would need another mutable context to
        // look up the mythic item.
        let mut mythic_to_masterwork_assoc: Vec<(String, String)> = Vec::new();

        ctx.items.borrow_mut().iter_mut().for_each(|(id, item)| {
            if item.is_masterwork {
                if let Some(ids) = item.from.as_ref() {
                    // the base mythic item id is contained as the first element in "from" ids (from
                    // API).
                    // we repeat this value for easier accessibility from an API perspective
                    item.masterwork_from = ids.get(0).map(|id| id.to_owned());
                    mythic_to_masterwork_assoc.push((
                        item.masterwork_from.as_ref().unwrap().to_owned(),
                        id.to_owned(),
                    ));
                }
            }
        });

        // mythic => masterwork association
        mythic_to_masterwork_assoc
            .iter()
            .for_each(|(mythic_id, masterwork_id)| {
                if let Some(mythic) = ctx.items.borrow_mut().get_mut(mythic_id) {
                    mythic.masterwork_into = Some(masterwork_id.to_owned());
                }
            });
    }
}
