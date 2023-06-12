use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::item::Item;

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
    pub table: StatGVTable,
    compute_stats: Vec<String>,
    override_fns: HashMap<String, Rc<StatGVTableEntryOverrideFn>>,
}

impl StatGVTableComputer {
    /// Creates a new stat gold value table computer with the given items and a vector of the stat
    /// names to compute.
    pub fn new(items: Rc<RefCell<HashMap<String, Item>>>, compute_stats: Vec<String>) -> Self {
        Self {
            items,
            table: StatGVTable::new(),
            compute_stats,
            override_fns: HashMap::new(),
        }
    }

    pub fn add_override_fn(
        &mut self,
        stat: &str,
        override_fn: Rc<StatGVTableEntryOverrideFn>,
    ) -> &mut Self {
        self.override_fns.insert(stat.to_string(), override_fn);
        self
    }

    pub fn compute(&mut self) {
        for stat in self.compute_stats.iter() {
            let entry = if let Some(override_fn) = self.override_fns.get(stat) {
                override_fn(self.items.clone())
            } else {
                self.compute_entry(stat)
            };

            self.table.insert(stat.to_string(), entry);
        }
    }

    fn compute_entry(&self, stat: &str) -> StatGVTableEntry {
        let mut lowest_value_item_id = ""; // the lowest value item id containing the stat
        let mut lowest_value_item_cost = 0; // the cost of the above
        let mut lowest_value_item_modifier = 0; // the value of the stat the above item gives

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
                if item.gold.purchasable && item.gold.base < lowest_value_item_cost {
                    lowest_value_item_id = id;
                    lowest_value_item_cost = item.gold.base;
                    // lowest_value_item_modifier = value.to_owned(); // todo: fix to use owned f64
                    // instead of &f64
                }
            }
        });

        if lowest_value_item_cost == 0 {
            StatGVTableEntry {
                unit_value: 0f64,
                computed_from_item_id: "n/a".to_owned(),
                notes: vec!["Could not associate stat to any item".to_string()],
            };
        } else {
            // todo: fix types
            // StatGVTableEntry {
            //     unit_value: lowest_value_item_modifier / lowest_value_item_cost,
            //     computed_from_item_id: lowest_value_item_id.to_owned(),
            //     notes: Vec::new()
            // }
        }

        todo!();
    }
}
