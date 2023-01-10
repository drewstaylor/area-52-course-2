
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

// XXX TODO: This is currently invalid / insecure. It takes 
// a Traveler as an argument (instead of loading from storage
// based on an address key). Like asking the enduser: "is 'True' 
// equal to 'True'", instead of checking their DNA actually
pub fn jump_ring_check(traveler: Traveler) -> StdResult<Binary> {
    let out = to_binary(&JumpRingCheckResponse {
        valid: traveler.cyberdized,
    })?;
    Ok(out)
}
