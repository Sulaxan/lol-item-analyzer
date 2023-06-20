use crate::data::transform::Transformer;

use super::ItemTransformContext;

/// Computes the given value of masterwork items.
/// ItemValueTransformer and MasterworkAssociatorTransformer must be run before this transformer
/// is run.
pub struct MasterworkItemValueTransformer;

impl Transformer<ItemTransformContext> for MasterworkItemValueTransformer {
    fn transform(&self, ctx: &mut ItemTransformContext) {
        // the gold value of masterwork items
        // vector of (id, gold_value)
        let mut masterwork_values: Vec<(String, f64)> = Vec::new();

        for (id, item) in ctx.items.borrow().iter() {
            if item.is_masterwork {
                if let Some(base_mythic_id) = item.masterwork_from.clone() {
                    if let Some(base_mythic) = ctx.items.borrow().get(&base_mythic_id) {
                        // The value of the masterwork item should simply be the difference
                        // between the masterwork item value and its base mythic value.
                        // The produced value *should* always be positive (by design of masterwork
                        // items).
                        let gold_value_difference = item.gold_value - base_mythic.gold_value;
                        masterwork_values.push((id.to_owned(), gold_value_difference));
                    }
                }
            }
        }

        // update the masterwor items
        for (id, gold_value) in masterwork_values.iter() {
            if let Some(masterwork_item) = ctx.items.borrow_mut().get_mut(id) {
                masterwork_item.masterwork_additional_gold_value = gold_value.clone();
            }
        }
    }
}
