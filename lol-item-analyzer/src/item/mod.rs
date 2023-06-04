use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod transformer;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rune {
    #[serde(rename = "isrune")]
    is_rune: bool,
    tier: u32,
    #[serde(rename = "type")]
    rune_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gold {
    base: u32,
    total: u32,
    sell: u32,
    purchasable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    full: String,
    sprite: String,
    group: String,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

type Stats = HashMap<String, f64>;

type Maps = HashMap<String, bool>;

type ItemIds = Vec<String>;

type Tags = Vec<String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    // id needs to be added in manually, since the item object in the API response does not contain
    // the id directly in the object, but rather is contained in the map of all the items.
    #[serde(skip_deserializing)]
    id: String,
    name: String,
    rune: Option<Rune>,
    gold: Gold,
    group: Option<String>,
    description: String,
    colloq: String,
    #[serde(rename = "plaintext")]
    plain_text: String,
    consumed: Option<bool>,
    stacks: Option<u32>,
    depth: Option<u32>,
    #[serde(rename = "consumeOnFull")]
    consume_on_full: Option<bool>,
    from: Option<ItemIds>,
    into: Option<ItemIds>,
    image: Image,
    #[serde(rename = "specialRecipe")]
    special_recipe: Option<u32>,
    #[serde(rename = "inStore")]
    in_store: Option<bool>,
    #[serde(rename = "hideFromAll")]
    hide_from_all: Option<bool>,
    #[serde(rename = "requiredChampion")]
    required_champion: Option<String>,
    #[serde(rename = "requiredAlly")]
    required_ally: Option<String>,
    stats: Stats,
    tags: Tags,
    maps: Maps,

    // Custom fields (not given by the LoL API, but computed later on)
    /// Whether this is an Ornn item
    #[serde(skip_deserializing)]
    is_masterwork: bool,
    /// The base item id if this is a masterwork item
    #[serde(skip_deserializing)]
    masterwork_from: String,
    /// The masterwork item id if this is the base item (assuming it is mythic)
    #[serde(skip_deserializing)]
    masterwork_into: Option<String>,
    #[serde(skip_deserializing)]
    gold_value: f64,
    /// The value given by the Ornn item
    #[serde(skip_deserializing)]
    masterwork_additional_gold_value: f64,
}
