#![cfg(test)]
use crate::{
    contract::{instantiate, query},
    msg::{InstantiateMsg, QueryMsg},
};
use cosmwasm_std::{Addr, from_binary};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
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
        visa_contract: Addr::unchecked("archway1yvnw8xj5elngcq95e2n2p8f80zl7shfwyxk88858pl6cgzveeqtqy7xtf7"),
        potion_contract: Addr::unchecked("archway1u6clujjm2qnem09gd4y7hhmulftvlt6mej4q0dd742tzcnsstt2q70lpu6"),
    };

    instantiate(deps.as_mut(), env.clone(), info, init_msg).unwrap();
    let res = query(deps.as_ref(), env.clone(), QueryMsg::MinimumSapience {}).unwrap();
    let res: SapienceResponse = from_binary(&res).unwrap();
    assert_eq!(res.level, ms_input);
}