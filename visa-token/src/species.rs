use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Species {
    pub name: String,
    pub sapience_level: SapienceScale,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub enum SapienceScale {
    None = 0,   // bugs
    Low = 1,    // cats, dogs
    Medium = 2, // ravens, rats, Terran humans
    High = 3,   // proper intelligent beings
}

impl SapienceScale {
    pub fn as_str(&self) -> &str {
        match self {
            SapienceScale::None => "None",
            SapienceScale::Low => "Low",
            SapienceScale::Medium => "Medium",
            SapienceScale::High => "High",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Traveler {
    pub name: String,
    pub home: Addr,
    pub species: Species,
    pub cyberdized: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SapienceResponse {
    pub level: SapienceScale,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Sapient {
    name: String,
    telepathic: bool,
}