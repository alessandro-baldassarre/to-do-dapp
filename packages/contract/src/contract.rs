use cosmwasm_schema::{cw_serde, QueryResponses};
use cw_utils::Expiration;

#[cw_serde]
pub struct ToDoResponse {
    pub task_id: u64,
    pub task: String,
    pub expiration: Expiration,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    AddToDo {
        task: String,
        expiration: Option<Expiration>,
    },
    DeleteToDo {
        task_id: u64,
    },
    UpdateToDo {
        task_id: u64,
        updated_task: Option<String>,
        expiration: Option<Expiration>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(bool)]
    AmIAdmin { addr: String },
    #[returns(GetToDoResponse)]
    GetToDo { task_id: u64 },
    #[returns(GetList)]
    GetList {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetToDoResponse {
    pub task_id: u64,
    pub task: String,
    pub expiration: Expiration,
}

#[cw_serde]
pub struct GetList {
    pub tasks: Vec<ToDoResponse>,
}
