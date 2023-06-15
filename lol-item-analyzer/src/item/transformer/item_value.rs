use std::{cell::RefCell, rc::Rc};

use crate::item::compute::{item_gv_calculator::ItemGVCalculator, stat_gv::StatGVTableComputer};

use super::{TransformContext, Transformer};

/// Gives items a gold value.
pub struct ItemValueTransformer {
    compute_stats: Vec<String>,
}

impl ItemValueTransformer {
    pub fn new(compute_stats: Vec<String>) -> Self {
        ItemValueTransformer { compute_stats }
    }
}

impl Transformer for ItemValueTransformer {
    fn transform(&self, ctx: &mut TransformContext) {
        let table_computer = StatGVTableComputer::new(ctx.items.clone(), self.compute_stats.clone());
        let table = table_computer.compute();
        let gv_calc = ItemGVCalculator::new(Rc::new(RefCell::new(table)));

        ctx.items.borrow_mut().iter_mut().for_each(|(_id, item)| {
            let value = gv_calc.get_value(item);
            item.gold_value = value.value;
        });
    }
}
