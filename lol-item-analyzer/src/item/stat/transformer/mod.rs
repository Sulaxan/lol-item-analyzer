use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::data::transform::Transformer;

use super::Stat;

pub mod percent_transformer;

pub struct StatTransformContext {
    pub stats: Rc<RefCell<HashMap<String, Stat>>>,
}

impl StatTransformContext {
    pub fn new(stats: HashMap<String, Stat>) -> Self {
        StatTransformContext {
            stats: Rc::new(RefCell::new(stats)),
        }
    }
}

#[derive(Clone)]
pub struct StatTransformHandler {
    pub stats: Vec<String>,
    pub transformers: Vec<Rc<RefCell<dyn Transformer<StatTransformContext>>>>,
}

impl StatTransformHandler {
    pub fn new(stats: Vec<String>, transformers: Vec<Rc<RefCell<dyn Transformer<StatTransformContext>>>>) -> Self {
        Self {
            stats,
            transformers,
        }
    }

    /// Transforms all stats using all transforms, returning a new map of stats. Transforms are
    /// applied one-by-one, in the given order, on all stats, before the next transform is applied.
    pub fn transform_all(&self) -> HashMap<String, Stat> {
        let mut stats = HashMap::new();
        self.stats.iter().for_each(|stat_id| {
            stats.insert(stat_id.to_owned(), {
                let mut stat = Stat::default();
                stat.id = stat_id.to_owned();
                stat
            });
        });

        let mut ctx = StatTransformContext::new(stats);

        self.transformers
            .iter()
            .for_each(|t| t.borrow_mut().transform(&mut ctx));

        ctx.stats.take()
    }
}
