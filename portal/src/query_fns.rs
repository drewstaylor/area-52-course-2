use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{to_binary, Binary, Deps, StdResult};

use crate::{
    msg::JumpRingCheckResponse, 
    state::CONFIG
};

use visa_token::Metadata;
use universe::species::{SapienceResponse, Traveler};

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VisasResponse {
    pub visas: Vec<Metadata>,
}

pub fn minimum_sapience(deps: Deps) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    let out = to_binary(&SapienceResponse {
        level: config.minimum_sapience,
    })?;
    Ok(out)
}

pub fn jump_ring_check(traveler: Traveler) -> StdResult<Binary> {
    let out = to_binary(&JumpRingCheckResponse {
        valid: traveler.cyberdized,
    })?;
    Ok(out)
}
