use cosmwasm_schema::cw_serde;
use cw_utils::Expiration;

use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct ToDo {
    pub task_id: u64,
    pub task: String,
    pub expiration: Expiration,
}

pub const LIST: Map<u64, ToDo> = Map::new("to_do_list");
pub const LIST_SEQ: Item<u64> = Item::new("list_seq");
