use std::{cell::RefCell, rc::Rc};

use crate::item::compute::{item_gv_calculator::ItemGVCalculator, stat_gv::StatGVTable};

use super::{TransformContext, Transformer};

/// Gives items a gold value.
pub struct ItemValueTransformer {
    stat_gv_table: Rc<RefCell<StatGVTable>>,
}

impl ItemValueTransformer {
    pub fn new(stat_gv_table: Rc<RefCell<StatGVTable>>) -> Self {
        ItemValueTransformer { stat_gv_table }
    }
}

impl Transformer for ItemValueTransformer {
    fn transform(&self, ctx: &mut TransformContext) {
        let gv_calc = ItemGVCalculator::new(self.stat_gv_table.clone());

        ctx.items.borrow_mut().iter_mut().for_each(|(_id, item)| {
            let value = gv_calc.get_value(item);
            item.gold_value = value.value;
        });
    }
}
