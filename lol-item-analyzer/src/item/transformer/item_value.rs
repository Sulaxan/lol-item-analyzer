use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    data::transform::Transformer,
    item::{
        gv::{item::ItemGVCalculator, stat::StatGVTableGenerator},
        stat::Stat,
    },
};

use super::ItemTransformContext;

/// Gives items a gold value.
pub struct ItemValueTransformer {
    stats: Rc<RefCell<HashMap<String, Stat>>>,
}

impl ItemValueTransformer {
    pub fn new(stats: Rc<RefCell<HashMap<String, Stat>>>) -> Self {
        ItemValueTransformer { stats }
    }
}

impl Transformer<ItemTransformContext> for ItemValueTransformer {
    fn transform(&self, ctx: &mut ItemTransformContext) {
        let table_generator = StatGVTableGenerator::new(ctx.items.clone(), self.stats.clone());
        let gv_table = table_generator.generate();
        let gv_calc = ItemGVCalculator::new(Rc::new(RefCell::new(gv_table)));

        ctx.items.borrow_mut().iter_mut().for_each(|(_id, item)| {
            let value = gv_calc.get_value(item);
            item.gold_value = value.value;
        });
    }
}
