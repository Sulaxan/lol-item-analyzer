use std::{cell::RefCell, collections::HashMap, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::item::{stat::Stat, Item};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StatGVTableEntry {
    /// The amount of gold a singular stat is worth.
    pub unit_value: f64,
    /// The item this gold value was computed from.
    pub computed_from_item_id: String,
    /// Specific computation notes created while computing a stat's gold value. Note this is mostly
    /// populated by custom manual computations.
    pub notes: Vec<String>,
}

pub type StatGVTable = HashMap<String, StatGVTableEntry>;

/// Function that takes in a map of items and produces a table entry.
pub type StatGVTableEntryOverrideFn =
    dyn Fn(Rc<RefCell<HashMap<String, Item>>>) -> StatGVTableEntry;

pub struct StatGVTableGenerator {
    pub items: Rc<RefCell<HashMap<String, Item>>>,
    compute_stats: Rc<RefCell<HashMap<String, Stat>>>,
    override_fns: HashMap<String, Rc<StatGVTableEntryOverrideFn>>,
}

impl StatGVTableGenerator {
    /// Creates a new stat gold value table generator with the given items and a vector of the
    /// stats to compute.
    pub fn new(
        items: Rc<RefCell<HashMap<String, Item>>>,
        compute_stats: Rc<RefCell<HashMap<String, Stat>>>,
    ) -> Self {
        Self {
            items,
            compute_stats,
            override_fns: HashMap::new(),
        }
    }

    pub fn add_override_fn(
        mut self,
        stat: &str,
        override_fn: Rc<StatGVTableEntryOverrideFn>,
    ) -> Self {
        self.override_fns.insert(stat.to_string(), override_fn);
        self
    }

    pub fn generate(&self) -> StatGVTable {
        let mut table = StatGVTable::new();

        for (id, stat) in self.compute_stats.borrow().iter() {
            let entry = if let Some(override_fn) = self.override_fns.get(id) {
                override_fn(self.items.clone())
            } else {
                self.compute_entry(stat)
            };

            table.insert(id.to_owned(), entry);
        }

        table
    }

    fn compute_entry(&self, stat: &Stat) -> StatGVTableEntry {
        // the lowest value item id containing the stat
        // tuple of (id, cost, modifier), where:
        // - id is the item id
        // - cost is the item cost
        // - modifier is the value of the stat the item gives
        let mut lowest_value_item: Option<(String, u32, f64)> = None;

        self.items.borrow().iter().for_each(|(id, item)| {
            if let Some(value) = item.stats.get(&stat.id) {
                // Base value isn't used since it only takes into account the final additional cost
                // of crafting the item after the player has all the components. Using the base
                // value is a bit misleading since it can inflate the cost of most stats. For some
                // stats, it may make sense using the base value since that stat is only available
                // in the final component (should override the stat calculation using an
                // override_fn in this case).
                if item.gold.purchasable
                    && (lowest_value_item == None
                        || item.gold.total < lowest_value_item.as_ref().unwrap().1)
                {
                    lowest_value_item = Some((id.to_owned(), item.gold.total, value.to_owned()));
                }
            }
        });

        if let Some((id, cost, modifier)) = lowest_value_item {
            let modifier = modifier * if stat.is_percent { 100f64 } else { 1f64 };

            StatGVTableEntry {
                unit_value: (cost as f64) / modifier,
                computed_from_item_id: id,
                notes: Vec::new(),
            }
        } else {
            StatGVTableEntry {
                unit_value: 0f64,
                computed_from_item_id: "n/a".to_owned(),
                notes: vec!["Could not associate stat to any item".to_string()],
            }
        }
    }
}
