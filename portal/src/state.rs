use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::msg::Visa;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use universe::species::{SapienceScale, Sapient};

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub planet_name: String,
    pub planet_sapients: Vec<Sapient>,
    pub minimum_sapience: SapienceScale,
}

pub const CONFIG: Item<Config> = Item::new("config");

/// Stored as visa_holder
pub const VISAS: Map<&Addr, Visa> = Map::new("visas");