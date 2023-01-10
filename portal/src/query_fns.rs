
use cosmwasm_std::{
    to_binary, Binary, Deps, StdResult,
};
use crate::{
    msg::JumpRingCheckResponse, state::CONFIG,
};
use universe::species::{SapienceResponse, Traveler};

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
