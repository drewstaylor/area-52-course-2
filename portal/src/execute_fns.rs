use cosmwasm_std::{
    Addr, CosmosMsg, DepsMut, Env, MessageInfo, QueryRequest, 
    to_binary, Response, WasmMsg, WasmQuery,
};

use crate::msg::MintMsg;
use cw721::{NftInfoResponse, TokensResponse};
use passport_token::{
    ExecuteMsg as Cw721ExecuteMsg, Extension, Metadata, 
    MintMsg as Cw721MintMsg, QueryMsg as Cw721QueryMsg,
};
use universe::species::{SapienceScale, Sapient};

use crate::{
    error::ContractError,
    state::CONFIG,
};

pub fn mint_passport(
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

    // Minting fails if user already owns a valid passport
    let query_msg: passport_token::QueryMsg<Extension> = Cw721QueryMsg::Tokens {
        owner: msg.identity.clone().into(),
        start_after: None,
        limit: None,
    };
    let query_req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: config.passport_contract.clone().into(),
        msg: to_binary(&query_msg).unwrap(),
    });
    let query_resp: TokensResponse = deps.querier.query(&query_req)?;
    if !query_resp.tokens.is_empty() {
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

    let mint_msg: passport_token::ExecuteMsg = Cw721ExecuteMsg::Mint(Cw721MintMsg {
        token_id: msg.identity.clone().into(),
        owner: msg.identity.into(),
        token_uri: None,
        extension: metadata_extension,
    });

    let mint_resp: CosmosMsg = WasmMsg::Execute {
        contract_addr: config.passport_contract.into(),
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

pub fn initiate_jump_ring_travel(
    _to: Addr,
    traveler: Addr,
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Verify traveler's passport
    let query_msg: passport_token::QueryMsg<Extension> = Cw721QueryMsg::NftInfo {
        token_id: traveler.clone().into(),
    };
    let query_req = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: config.passport_contract.clone().into(),
        msg: to_binary(&query_msg).unwrap(),
    });
    let query_resp: NftInfoResponse<Metadata> = deps.querier.query(&query_req)?;

    // Since we're using soulbound NFTs, and because only the JumpRing contract 
    // can mint, identity theft shouldn't be possible. We'll check just in case
    // but the below statement could probably be safely removed from the project
    // since if the token didn't exist the contract call would fail with an error
    // in the preceding line (e.g. deps.querier.query(&query_req)?)
    if query_resp.extension.identity.unwrap().to_string() != traveler.clone().to_string() {
        return Err(ContractError::Unauthorized {});
    }
    
    // XXX TODO: Process JumpRing travel -> _to: Addr

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
