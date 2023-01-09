use cosmwasm_std::{
    from_binary, to_binary, Addr, DepsMut, Env, MessageInfo, QueryRequest, Response, WasmQuery,
};

use cw721::{Cw721QueryMsg, NftInfoResponse};
use visa_token::Metadata;
use universe::species::{SapienceResponse, SapienceScale, Sapient};

use crate::{
    error::ContractError,
    msg::{AssignVisaMsg, Visa},
    query_fns::minimum_sapience,
    state::{CONFIG, VISAS},
};

// XXX TODO: 
// The current functions for handling NFTs can be removed wholesale
// and be safely replaced by a check in `initiate_jump_ring_travel` that verifies
// the caller owns an NFT of the visa-token contract. Additionally, the following
// additions will be required:
//
// 1) Minting endpoint in `execute_fns.rs`
// 2) Minting endpoint must enforce users can only hold one NFT from the token contract
// 3) E.g. users may mint if they never minted; or, if they've burned their token

// XXX TODO: isn't needed
pub fn receive_visa(
    sender: String,
    token_id: String,
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let msg = Cw721QueryMsg::NftInfo {
        token_id: token_id.clone(),
    };

    let query = WasmQuery::Smart {
        contract_addr: info.sender.to_string(), // XXX TODO: Fix this error by loading Visa contract addr from state
        msg: to_binary(&msg)?,
    };

    let res: NftInfoResponse<Metadata> = deps.querier.query(&QueryRequest::Wasm(query))?;

    let extension_metadata = res.extension;
    let incoming_sapience_level = extension_metadata.sapience.unwrap();
    
    let contract_min_sapience: SapienceResponse = from_binary(
        &minimum_sapience(deps.as_ref()).unwrap()
    ).unwrap();

    if incoming_sapience_level.as_num() < contract_min_sapience.level.as_num() {
        return Err(ContractError::NotSmartEnough {});
    }

    VISAS.update(deps.storage, &Addr::unchecked(sender), |old| match old {
        None => Err(ContractError::NotOnList {}),
        Some(mut visa) => {
            visa.approved = true;
            Ok(visa)
        }
    })?;

    Ok(Response::new()
        .add_attribute("action", "receive_visa")
        .add_attribute("new_owner", env.contract.address) // XXX TODO: Remove this as ownership is not actually transferred
        .add_attribute("new_token_id", token_id))
}

// XXX TODO: isn't needed
/// Receive initial details and add to visa whitelist for later verification.
pub fn assign_visa(
    msg: AssignVisaMsg,
    deps: DepsMut,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    // XXX TODO: Handle Visa approval (see above fn)

    let visa = Visa {
        approved: false,
        details: msg.details.clone(),
    };

    VISAS.save(deps.storage, &msg.details.holder, &visa)?;

    Ok(Response::new().add_attribute("action", "assign_visa"))
}

pub fn initiate_jump_ring_travel(
    _to: Addr,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {

    // XXX TODO: replace this with a query to `visa_token::tokens` to check if info.sender owns an NFT
    let visa = match VISAS.load(deps.storage, &info.sender) {
        Ok(v) => v,
        Err(_) => return Err(ContractError::NotOnList {}),
    };

    // XXX TODO: isn't needed
    if !visa.approved {
        return Err(ContractError::Unapproved {});
    }

    Ok(Response::new()
        .add_attribute("action", "initiate_jump_ring_travel")
        .add_attribute("traveler", &info.sender))
}

pub fn set_minimum_sapience(
    to: SapienceScale,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    config.minimum_sapience = to;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::default())
}

pub fn set_planet_name(
    to: String,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }
    config.planet_name = to;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("action", "set_planet_name"))
}

pub fn set_sapient_names(
    to: Vec<Sapient>,
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    config.planet_sapients = to;
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attribute("action", "set_sapient_names"))
}
