use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod loader;

#[derive(Serialize, Deserialize)]
struct Rune {
    #[serde(rename = "isrune")]
    is_rune: bool,
    tier: u32,
    #[serde(rename = "type")]
    rune_type: String,
}

#[derive(Serialize, Deserialize)]
struct Gold {
    base: u32,
    total: u32,
    sell: u32,
    purchasable: bool,
}

#[derive(Serialize, Deserialize)]
struct Image {
    full: String,
    sprite: String,
    group: String,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

type Stats = HashMap<String, u32>;

type Maps = HashMap<String, bool>;

type ItemIds = Vec<u32>;

type Tags = Vec<String>;

#[derive(Serialize, Deserialize)]
struct Item {
    name: String,
    rune: Rune,
    gold: Gold,
    group: String,
    description: String,
    colloq: String,
    #[serde(rename = "plaintext")]
    plain_text: String,
    consumed: bool,
    stacks: u32,
    depth: u32,
    #[serde(rename = "consumeOnFull")]
    consume_on_full: bool,
    from: ItemIds,
    into: ItemIds,
    image: Image,
    #[serde(rename = "specialRecipe")]
    special_recipe: u32,
    #[serde(rename = "inStore")]
    in_store: bool,
    #[serde(rename = "hideFromAll")]
    hide_from_all: bool,
    #[serde(rename = "requiredChampion")]
    required_champion: String,
    #[serde(rename = "requiredAlly")]
    required_ally: String,
    stas: Stats,
    tags: Tags,
    maps: Maps,
}
