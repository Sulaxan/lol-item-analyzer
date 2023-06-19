use std::{cell::RefCell, rc::Rc};

use lol_item_analyzer::{
    data::lol_api::LolApi,
    item::{
        compute::stat_gv::StatGVTableGenerator,
        transformer::{
            id_assoc::IdAssociatorTransformer, item_value::ItemValueTransformer,
            masterwork_assoc::MasterworkAssociatorTransformer,
            masterwork_ident::MasterworkIdentifierTransformer,
            masterwork_item_value::MasterworkItemValueTransformer, TransformHandler,
        },
    },
};

#[tokio::test]
async fn generate_gv_table() {
    let latest_version = LolApi::get_latest_version().await.unwrap();
    let items = Rc::new(RefCell::new(
        LolApi::get_items(&latest_version).await.unwrap(),
    ));
    let stats = LolApi::get_stat_ids(&latest_version).await.unwrap();

    let table_computer = StatGVTableGenerator::new(items.clone(), stats);
    let table = table_computer.compute();

    println!("{:#?}", table);
}

#[tokio::test]
async fn everything_works() {
    let latest_version = LolApi::get_latest_version().await.unwrap();
    let items = LolApi::get_items(&latest_version).await.unwrap();
    let stats = LolApi::get_stat_ids(&latest_version).await.unwrap();
    println!("Stats: {:#?}", stats);

    let transformer = TransformHandler::new(
        items,
        vec![
            Rc::new(RefCell::new(IdAssociatorTransformer)),
            Rc::new(RefCell::new(MasterworkIdentifierTransformer)),
            Rc::new(RefCell::new(MasterworkAssociatorTransformer)),
            Rc::new(RefCell::new(ItemValueTransformer::new(stats))),
            Rc::new(RefCell::new(MasterworkItemValueTransformer)),
        ],
    );

    let new_items = transformer.transform_all();

    let pickaxe_item = new_items.get("1037").unwrap();
    assert_eq!(pickaxe_item.is_masterwork, false);
    assert_eq!(pickaxe_item.gold_value, 875f64);

    let upgraded_aeropack_item = new_items.get("7011").unwrap();
    assert_eq!(upgraded_aeropack_item.is_masterwork, true);
    assert_eq!(
        upgraded_aeropack_item.masterwork_from.as_ref().unwrap(),
        "3152"
    );

    println!(
        "Total value of Upgraded Aeropack: {}",
        upgraded_aeropack_item.gold_value
    );
    println!(
        "Masterwork value of Upgraded Aeropack: {}",
        upgraded_aeropack_item.masterwork_additional_gold_value
    );
}
