use crate::msg::ResponseStatus::Success;
use crate::msg::{HandleAnswer, HandleMsg, InitMsg, QueryAnswer, QueryMsg};
use crate::state::{config, config_read, State};
use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse, Querier,
    StdResult, Storage,
};
use secret_toolkit::snip20;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        viewing_key: msg.viewing_key,
    };

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::SetViewingKeyForSnip20 {
            address,
            contract_hash,
        } => set_viewing_key_for_snip20(deps, env, address, contract_hash),
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn query_config<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<QueryAnswer> {
    let state = config_read(&deps.storage).load()?;

    Ok(QueryAnswer::Config {
        viewing_key: state.viewing_key,
    })
}

fn set_viewing_key_for_snip20<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _env: Env,
    address: HumanAddr,
    contract_hash: String,
) -> StdResult<HandleResponse> {
    let state = config_read(&deps.storage).load()?;

    Ok(HandleResponse {
        messages: vec![snip20::set_viewing_key_msg(
            state.viewing_key,
            None,
            1,
            contract_hash,
            address,
        )?],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::SetViewingKeyForSnip20 {
            status: Success,
        })?),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::from_binary;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage};

    // === HELPERS ===
    fn init_helper() -> (
        StdResult<InitResponse>,
        Extern<MockStorage, MockApi, MockQuerier>,
    ) {
        let env = mock_env("user", &[]);
        let mut deps = mock_dependencies(20, &[]);
        let msg = InitMsg {
            viewing_key: "accepteverythingjusthewayitis".to_string(),
        };
        (init(&mut deps, env.clone(), msg), deps)
    }

    #[test]
    fn test_query_config() {
        let (_init_result, deps) = init_helper();
        let res = from_binary(&query(&deps, QueryMsg::Config {}).unwrap()).unwrap();
        match res {
            QueryAnswer::Config { viewing_key } => {
                assert_eq!(viewing_key, "accepteverythingjusthewayitis".to_string());
            }
        }
    }

    #[test]
    fn test_handle_set_viewing_key_for_snip20() {
        let (_init_result, mut deps) = init_helper();

        // = * It calls viewing key for snip 20
        let handle_msg = HandleMsg::SetViewingKeyForSnip20 {
            address: HumanAddr::from("token-address"),
            contract_hash: "token-contract-hash".to_string(),
        };
        let handle_result = handle(&mut deps, mock_env("user", &[]), handle_msg);
        let handle_result_unwrapped = handle_result.unwrap();
        assert_eq!(
            handle_result_unwrapped.messages,
            vec![snip20::set_viewing_key_msg(
                "accepteverythingjusthewayitis".to_string(),
                None,
                1,
                "token-contract-hash".to_string(),
                HumanAddr::from("token-address"),
            )
            .unwrap()],
        );
        let handle_result_data: HandleAnswer =
            from_binary(&handle_result_unwrapped.data.unwrap()).unwrap();
        assert_eq!(
            to_binary(&handle_result_data).unwrap(),
            to_binary(&HandleAnswer::SetViewingKeyForSnip20 { status: Success }).unwrap()
        );
    }
}
