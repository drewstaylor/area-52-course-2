#![cfg(test)]

use crate::{
    contract::{instantiate, query},
    error::ContractError,
    execute_fns::initiate_jump_ring_travel,
    msg::{InstantiateMsg, QueryMsg, Visa, VisaAdminDetails},
    state::VISAS,
};
use cosmwasm_std::{
    from_binary,
    testing::{mock_dependencies, mock_env},
};
use cosmwasm_std::{testing::mock_info, Addr, Response};
use cw_storage_plus::Item;
use universe::species::{SapienceResponse, SapienceScale, Sapient};

#[test]
pub fn check_minimum_sapience_level() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let sender_name = "not on list";
    let info = mock_info(sender_name, &[]);
    let ms_input = SapienceScale::High;

    // Set the minimum_sapience
    let init_msg = InstantiateMsg {
        planet_name: "foo".to_string(),
        planet_sapients: vec![Sapient {
            name: "cyborg".to_string(),
            telepathic: true,
        }],
        minimum_sapience: SapienceScale::High,
    };

    instantiate(deps.as_mut(), env.clone(), info, init_msg).unwrap();
    let res = query(deps.as_ref(), env.clone(), QueryMsg::MinimumSapience {}).unwrap();
    let res: SapienceResponse = from_binary(&res).unwrap();
    assert_eq!(res.level, ms_input);
}

#[test]
pub fn visa_not_on_list() {
    let mut deps = mock_dependencies();
    let sender_name = "not on list";
    let sender = mock_info(sender_name, &[]);
    let dest = Addr::unchecked("mars");

    let err = initiate_jump_ring_travel(dest, deps.as_mut(), sender).unwrap_err();
    assert_eq!(err, ContractError::NotOnList {})
}

#[allow(dead_code)]
pub fn item_test() {
    let mut deps = mock_dependencies();

    let game_name: Item<String> = Item::new("cfg_info");
    game_name
        .save(deps.as_mut().storage, &"Race for the Galaxy".to_string())
        .unwrap();
}
#[test]
pub fn visa_is_approved() {
    let mut deps = mock_dependencies();
    let info = mock_info("zeus", &[]);
    let dest = Addr::unchecked("mars");

    let details = VisaAdminDetails {
        ape: Addr::unchecked("ape"),
        contract: Addr::unchecked("mars"),
        holder: Addr::unchecked("mars"),
        token_id: "dakkadakka".to_string(),
    };
    let visa = Visa {
        approved: true,
        details: details,
    };
    VISAS.save(&mut deps.storage, &info.sender, &visa).unwrap();
    let res = initiate_jump_ring_travel(dest, deps.as_mut(), info.clone()).unwrap();
    assert_eq!(
        res,
        Response::new()
            .add_attribute("action", "initiate_jump_ring_travel")
            .add_attribute("traveler", &info.sender)
    )
}
