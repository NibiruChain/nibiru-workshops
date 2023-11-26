use cosmwasm_schema::cw_serde;
use serde::{Deserialize, Serialize};

use crate::state::TodoTask;

#[cw_serde]
pub struct InstantiateMsg {
    pub todos: Option<Vec<String>>,
}

#[cw_serde]
pub enum ExecuteMsg {
    Add { task: String },

    Toggle { task_id: u64 },

    Delete { task_id: u64 },
}

#[cw_serde]
#[derive(cosmwasm_schema::QueryResponses)]
pub enum QueryMsg {
    /// Fetches all of the tasks
    #[returns(Vec<TodoTask>)]
    Todos {},

    /// Fetch single task with the given ID.
    #[returns(TodoTask)]
    TodoTask { id: u64 },

    /// Fetch all tasks marked as done after the given `since_id`.
    #[returns(Vec<TodoTask>)]
    CompletedTasks { since_id: Option<u64> },
}
