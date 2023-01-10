use cosmwasm_std::{
    Addr, CosmosMsg, DepsMut, Env, MessageInfo, to_binary, 
    Response, WasmMsg,
};

use crate::msg::MintMsg;
// use cw721::{Cw721QueryMsg, NftInfoResponse};
use visa_token::{ExecuteMsg as Cw721ExecuteMsg, Extension, Metadata, MintMsg as Cw721MintMsg};
use universe::species::{SapienceScale, Sapient};

use crate::{
    error::ContractError,
    state::CONFIG,
};

pub fn mint_visa(
    msg: MintMsg,
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    
    // Only potion contract can call this function
    let potion_contract = config.potion_contract;
    if info.sender != potion_contract {
        return Err(ContractError::Unauthorized {});
    }

    let metadata_extension: Extension = Some(Metadata {
        name: Some(msg.name),
        description: Some(msg.description),
        image: Some(msg.image),
        dna: Some(msg.dna), // XXX TODO (drew): Re-work the way DNA strings are built and parsed in Potion contract
        species: Some(msg.species),
        sapience_level: Some(msg.sapience_level),
        issuer: Some(env.contract.address.clone()),
        origin: Some(env.contract.address),
        identity: Some(msg.identity.clone()),
    });

    let mint_msg: visa_token::ExecuteMsg = Cw721ExecuteMsg::Mint(Cw721MintMsg {
        token_id: msg.identity.clone().into(),
        owner: msg.identity.into(),
        token_uri: None,
        extension: metadata_extension,
    });

    let mint_resp: CosmosMsg = WasmMsg::Execute {
        contract_addr: config.visa_contract.into(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    }
    .into();

    // When calling another contract we need to use a vector of responses
    // This allows for returning separate responses for state transitions
    // in both contracts
    let mut messages = Vec::new();
    messages.push(mint_resp);
    Ok(Response::new().add_messages(messages))
}

// XXX TODO: 
// The following additions are required:
//
// 1) DONE - Minting endpoint in `execute_fns.rs`
// 2) Minting endpoint must enforce users can only hold one NFT from the token contract
// 3) E.g. users may mint if they never minted; or, if they've burned their token

pub fn initiate_jump_ring_travel(
    _to: Addr,
    traveler: Addr,
    _deps: DepsMut,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    // XXX TODO: Verify Visa here

    Ok(Response::new()
        .add_attribute("action", "initiate_jump_ring_travel")
        .add_attribute("traveler", &traveler))
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
