use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;

use universe::species::{SapienceScale, Sapient, Traveler};

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AssignVisaMsg {
    pub details: VisaAdminDetails,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VisaAdminDetails {
    pub ape: Addr,
    /// The previous contract this is being sent from.
    pub contract: Addr,
    pub holder: Addr,
    /// The token_id of the Visa to be approved later.
    pub token_id: String,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Visa {
    pub approved: bool,
    pub details: VisaAdminDetails,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    JumpRingPreCheck { traveler: Traveler },
    MinimumSapience {},
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    SetPlanetName { to: String },
    SetSapientNames { to: Vec<Sapient> },
    SetMinimumSapience { to: SapienceScale },
    JumpRingTravel { to: Addr },
    AssignVisa { msg: AssignVisaMsg },
    ReceiveNft(cw721::Cw721ReceiveMsg),
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub planet_name: String,
    pub planet_sapients: Vec<Sapient>,
    pub minimum_sapience: SapienceScale,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JumpRingCheckResponse {
    pub valid: bool,
}
