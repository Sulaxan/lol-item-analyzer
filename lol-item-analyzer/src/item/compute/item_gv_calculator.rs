use std::{cell::RefCell, rc::Rc};

use crate::item::Item;

use super::stat_gv::StatGVTable;

pub enum ItemGVTaskResult {
    /// The stat and the computed value for the stat
    ComputedStat(String, f64),
    /// The stat not computed
    NotComputed(String),
}

pub struct ItemGVResult {
    /// The total computed value of the item
    pub value: f64,
    /// The list of work done while computing the total value
    pub compute_task_results: Vec<ItemGVTaskResult>,
}

pub struct ItemGVCalculator {
    pub stat_gv_table: Rc<RefCell<StatGVTable>>,
}

impl ItemGVCalculator {
    pub fn new(stat_gv_table: Rc<RefCell<StatGVTable>>) -> Self {
        ItemGVCalculator { stat_gv_table }
    }

    /// Get the gold value for an item
    pub fn get_value(&self, item: &Item) -> ItemGVResult {
        let mut value = 0f64;
        let mut task_statuses = Vec::new();

        item.stats.iter().for_each(|(stat, amount)| {
            if let Some(entry) = self.stat_gv_table.borrow().get(stat) {
                let stat_value = entry.unit_value * amount;
                value += stat_value;
                task_statuses.push(ItemGVTaskResult::ComputedStat(stat.to_owned(), stat_value));
            } else {
                task_statuses.push(ItemGVTaskResult::NotComputed(stat.to_owned()));
            }
        });

        ItemGVResult {
            value,
            compute_task_results: task_statuses,
        }
    }
}
