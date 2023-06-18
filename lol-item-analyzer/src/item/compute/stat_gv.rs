use std::{cell::RefCell, collections::HashMap, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::item::Item;

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

pub struct StatGVTableComputer {
    pub items: Rc<RefCell<HashMap<String, Item>>>,
    compute_stats: Vec<String>,
    override_fns: HashMap<String, Rc<StatGVTableEntryOverrideFn>>,
}

impl StatGVTableComputer {
    /// Creates a new stat gold value table computer with the given items and a vector of the stat
    /// names to compute.
    pub fn new(items: Rc<RefCell<HashMap<String, Item>>>, compute_stats: Vec<String>) -> Self {
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

    pub fn add_compute_stat(mut self, stat: &str) -> Self {
        self.compute_stats.push(stat.to_owned());
        self
    }

    pub fn compute(&self) -> StatGVTable {
        let mut table = StatGVTable::new();

        for stat in self.compute_stats.iter() {
            let entry = if let Some(override_fn) = self.override_fns.get(stat) {
                override_fn(self.items.clone())
            } else {
                self.compute_entry(stat)
            };

            table.insert(stat.to_string(), entry);
        }

        table
    }

    fn compute_entry(&self, stat: &str) -> StatGVTableEntry {
        // the lowest value item id containing the stat
        // tuple of (id, cost, modifier), where:
        // - id is the item id
        // - cost is the item cost
        // - modifier is the value of the stat the item gives
        let mut lowest_value_item: Option<(String, u32, f64)> = None;

        self.items.borrow().iter().for_each(|(id, item)| {
            if let Some(value) = item.stats.get(stat) {
                // we use the base gold value (which is the cost of the item itself, excluding
                // the price of previously purchased parts) as it should lead to more accurate
                // computations for a stat
                // e.g.,
                // some item alpha is built using item beta
                // => item alpha = 550g, item beta = 250g
                // => item beta gets 400 HP, while item alpha gets 400 HP + 15% move speed
                // => using base value of item alpha (300g, the craft cost), allows us to compute
                // the value of 1% move speed given that 300g = 15% move speed
                //
                // * note: there may be some inaccuracies with this, but this way of computing stat
                // gold value should work for most stats
                //
                // NOTE: TEMPORARILY CHANGED TO USE TOTAL GOLD
                if item.gold.purchasable
                    && (lowest_value_item == None
                        || item.gold.total < lowest_value_item.as_ref().unwrap().1)
                {
                    lowest_value_item = Some((id.to_owned(), item.gold.total, value.to_owned()));
                }
            }
        });

        if let Some((id, cost, modifier)) = lowest_value_item {
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
