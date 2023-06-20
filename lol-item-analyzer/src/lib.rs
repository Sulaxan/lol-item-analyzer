use std::{cell::RefCell, collections::HashMap, rc::Rc};

use data::transform::Transformer;
use item::{
    stat::transformer::{StatTransformContext, StatTransformHandler},
    transformer::{ItemTransformContext, TransformHandler},
    Item,
};

use crate::item::{
    stat::transformer::percent_transformer::StatPercentTransformer,
    transformer::{
        id_assoc::IdAssociatorTransformer, item_value::ItemValueTransformer,
        masterwork_assoc::MasterworkAssociatorTransformer,
        masterwork_ident::MasterworkIdentifierTransformer,
        masterwork_item_value::MasterworkItemValueTransformer,
    },
};

pub mod data;
pub mod item;

#[derive(Clone)]
pub struct StatAnalyzer {
    pub transform_handler: StatTransformHandler,
}

impl StatAnalyzer {
    pub fn new(
        stats: Vec<String>,
        transformers: Vec<Rc<RefCell<dyn Transformer<StatTransformContext>>>>,
    ) -> Self {
        Self {
            transform_handler: StatTransformHandler::new(stats, transformers),
        }
    }

    pub fn default_transformers() -> Vec<Rc<RefCell<dyn Transformer<StatTransformContext>>>> {
        vec![Rc::new(RefCell::new(StatPercentTransformer))]
    }
}

#[derive(Clone)]
pub struct ItemAnalyzer {
    pub transform_handler: TransformHandler,
}

impl ItemAnalyzer {
    pub fn new(
        items: HashMap<String, Item>,
        transformers: Vec<Rc<RefCell<dyn Transformer<ItemTransformContext>>>>,
    ) -> Self {
        Self {
            transform_handler: TransformHandler::new(items, transformers),
        }
    }

    pub fn default_transformers(
        stat_analyzer: &StatAnalyzer,
    ) -> Vec<Rc<RefCell<dyn Transformer<ItemTransformContext>>>> {
        let stats = Rc::new(RefCell::new(
            stat_analyzer.transform_handler.transform_all(),
        ));

        let transformers: Vec<Rc<RefCell<dyn Transformer<ItemTransformContext>>>> = vec![
            Rc::new(RefCell::new(IdAssociatorTransformer)),
            Rc::new(RefCell::new(MasterworkIdentifierTransformer)),
            Rc::new(RefCell::new(MasterworkAssociatorTransformer)),
            Rc::new(RefCell::new(ItemValueTransformer::new(stats))),
            Rc::new(RefCell::new(MasterworkItemValueTransformer)),
        ];

        transformers
    }
}
