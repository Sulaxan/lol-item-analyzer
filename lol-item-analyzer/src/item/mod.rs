use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod transformer;
pub mod compute;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Rune {
    #[serde(rename = "isrune")]
    pub is_rune: bool,
    pub tier: u32,
    #[serde(rename = "type")]
    pub rune_type: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Gold {
    pub base: u32,
    pub total: u32,
    pub sell: u32,
    pub purchasable: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Image {
    pub full: String,
    pub sprite: String,
    pub group: String,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

pub type Stats = HashMap<String, f64>;

pub type Maps = HashMap<String, bool>;

pub type ItemIds = Vec<String>;

pub type Tags = Vec<String>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    // id needs to be added in manually, since the item object in the API response does not contain
    // the id directly in the object, but rather is contained in the map of all the items.
    #[serde(skip_deserializing)]
    pub id: String,
    pub name: String,
    pub rune: Option<Rune>,
    pub gold: Gold,
    pub group: Option<String>,
    pub description: String,
    pub colloq: String,
    #[serde(rename = "plaintext")]
    pub plain_text: String,
    pub consumed: Option<bool>,
    pub stacks: Option<u32>,
    pub depth: Option<u32>,
    #[serde(rename = "consumeOnFull")]
    pub consume_on_full: Option<bool>,
    pub from: Option<ItemIds>,
    pub into: Option<ItemIds>,
    pub image: Image,
    #[serde(rename = "specialRecipe")]
    pub special_recipe: Option<u32>,
    #[serde(rename = "inStore")]
    pub in_store: Option<bool>,
    #[serde(rename = "hideFromAll")]
    pub hide_from_all: Option<bool>,
    #[serde(rename = "requiredChampion")]
    pub required_champion: Option<String>,
    #[serde(rename = "requiredAlly")]
    pub required_ally: Option<String>,
    pub stats: Stats,
    pub tags: Tags,
    pub maps: Maps,

    // Custom fields (not given by the LoL API, but computed later on)
    /// Whether this is an Ornn item
    #[serde(skip_deserializing)]
    pub is_masterwork: bool,
    /// The base item id if this is a masterwork item
    #[serde(skip_deserializing)]
    pub masterwork_from: Option<String>,
    /// The masterwork item id if this is the base item (assuming it is mythic)
    #[serde(skip_deserializing)]
    pub masterwork_into: Option<String>,
    #[serde(skip_deserializing)]
    pub gold_value: f64,
    /// The value given by the Ornn item
    #[serde(skip_deserializing)]
    pub masterwork_additional_gold_value: f64,
}

impl Default for Item {
    fn default() -> Self {
        Item {
            id: "0000".to_string(),
            name: "name".to_string(),
            rune: None,
            gold: Gold::default(),
            group: None,
            description: "description".to_string(),
            colloq: ";".to_string(),
            plain_text: "".to_string(),
            consumed: None,
            stacks: None,
            depth: None,
            consume_on_full: None,
            from: None,
            into: None,
            image: Image::default(),
            special_recipe: None,
            in_store: None,
            hide_from_all: None,
            required_champion: None,
            required_ally: None,
            stats: Stats::new(),
            tags: Tags::new(),
            maps: Maps::new(),
            is_masterwork: false,
            masterwork_from: None,
            masterwork_into: None,
            gold_value: 0f64,
            masterwork_additional_gold_value: 0f64,
        }
    }
}
