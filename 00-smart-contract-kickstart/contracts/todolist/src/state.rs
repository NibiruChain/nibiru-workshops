use cosmwasm_schema::cw_serde;
use cw_storage_plus::{Item, Map};

pub const TASKS: Map<u64, TodoTask> = Map::new("tasks");
pub const NEXT_TASK_ID: Item<u64> = Item::new("next_task_id");

/// Represents an individual to-do task
#[cw_serde]
pub struct TodoTask {
    pub id: u64,
    pub description: String,
    pub done: bool,
}
