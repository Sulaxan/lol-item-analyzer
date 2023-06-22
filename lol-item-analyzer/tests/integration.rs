use std::{cell::RefCell, rc::Rc};

use lol_item_analyzer::{
    data::lol_api::LolApi, item::gv::stat::StatGVTableGenerator, ItemAnalyzer, StatAnalyzer,
};

#[tokio::test]
async fn generate_gv_table() {
    let latest_version = LolApi::get_latest_version().await.unwrap();
    let items = Rc::new(RefCell::new(
        LolApi::get_items(&latest_version).await.unwrap(),
    ));
    let raw_stats = LolApi::get_stat_ids(&latest_version).await.unwrap();

    let handler = StatAnalyzer::new(raw_stats, StatAnalyzer::default_transformers());
    let stats = Rc::new(RefCell::new(handler.transform_handler.transform_all()));

    let table_computer = StatGVTableGenerator::new(items.clone(), stats.clone());
    let table = table_computer.generate();

    println!("{:#?}", table);
}

#[tokio::test]
async fn everything_works() {
    let latest_version = LolApi::get_latest_version().await.unwrap();
    let raw_items = LolApi::get_items(&latest_version).await.unwrap();
    let raw_stats = LolApi::get_stat_ids(&latest_version).await.unwrap();
    println!("Stats: {:#?}", raw_stats);

    let stat_analyzer = StatAnalyzer::new(raw_stats, StatAnalyzer::default_transformers());
    let item_anaylzer = ItemAnalyzer::new(
        raw_items,
        ItemAnalyzer::default_transformers(&stat_analyzer),
    );

    let new_items = item_anaylzer.transform_handler.transform_all();

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
