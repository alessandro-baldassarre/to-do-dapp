#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;
use cw_controllers::Admin;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::LIST_SEQ;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:to-do-dapp";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin_addr = msg
        .admin
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?
        .unwrap_or(info.sender);

    let admin = Admin::new("admin");
    admin.set(deps.branch(), Some(admin_addr))?;

    LIST_SEQ.save(deps.storage, &0u64)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddToDo { task, expiration } => {
            execute::add_to_do(deps, env, info, task, expiration)
        }
        _ => unimplemented!(),
    }
}

pub mod execute {
    use std::ops::Add;

    use cosmwasm_std::ensure;
    use cw_utils::Expiration;

    use crate::state::{ToDo, LIST};

    use super::*;

    pub fn add_to_do(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        task: String,
        expiration: Option<Expiration>,
    ) -> Result<Response, ContractError> {
        let admin = Admin::new("admin");
        let admin_addr = admin.get(deps.as_ref())?.unwrap();

        ensure!(info.sender == admin_addr, ContractError::Unauthorized {});
        let task_id =
            LIST_SEQ.update::<_, cosmwasm_std::StdError>(deps.storage, |id| Ok(id.add(1)))?;
        let new_task = ToDo {
            task_id,
            task,
            expiration: expiration.unwrap_or(Expiration::Never {}),
        };

        LIST.save(deps.storage, task_id, &new_task)?;

        Ok(Response::new()
            .add_attribute("method", "add_to_do")
            .add_attribute("task_id", task_id.to_string()))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AmIAdmin { addr } => to_binary(&query::am_i_admin(deps, &addr)?),
        QueryMsg::GetToDo { task_id } => to_binary(&query::get_to_do(deps, task_id)?),
        // QueryMsg::GetList { start_after, limit } => unimplemented!(),
        _ => unimplemented!(),
    }
}

pub mod query {
    use cosmwasm_std::StdError;

    use crate::{msg::GetToDoResponse, state::LIST};

    use super::*;

    pub fn am_i_admin(deps: Deps, addr: &str) -> StdResult<bool> {
        let addr = deps.api.addr_validate(addr)?;
        let admin = Admin::new("admin");
        let res = admin.is_admin(deps, &addr)?;
        Ok(res)
    }

    pub fn get_to_do(deps: Deps, task_id: u64) -> StdResult<GetToDoResponse> {
        let task = LIST
            .may_load(deps.storage, task_id)?
            .ok_or(StdError::GenericErr {
                msg: "Task non found".to_owned(),
            })?;
        let res = GetToDoResponse {
            task_id: task.task_id,
            task: task.task,
            expiration: task.expiration,
        };
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::msg::GetToDoResponse;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};
    use cw_utils::Expiration;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            admin: Some("creator".to_owned()),
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let info = mock_info("creator", &coins(1000, "earth"));
        // it worked, let's query the state
        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::AmIAdmin {
                addr: info.sender.to_string(),
            },
        )
        .unwrap();
        let value: bool = from_binary(&res).unwrap();
        assert!(value);
    }

    #[test]
    fn add_task() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            admin: Some("creator".to_owned()),
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        // we can just call .unwrap() to assert this was a success
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("creator", &coins(1000, "earth"));
        let msg = ExecuteMsg::AddToDo {
            task: "write the best CosmWasm contract".to_owned(),
            expiration: None,
        };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase task_id by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetToDo { task_id: 1 }).unwrap();
        let value: GetToDoResponse = from_binary(&res).unwrap();
        assert_eq!(
            GetToDoResponse {
                task_id: 1,
                task: "write the best CosmWasm contract".to_owned(),
                expiration: Expiration::Never {}
            },
            value
        );
    }
}
