// use crate::add_coins;
use crate::error::ContractError;
use crate::events::{event_add_task, event_delete_task, event_toggle_task};
// use crate::events::{new_incentives_program_event, new_program_funding};
use crate::msgs::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{TodoTask, NEXT_TASK_ID, TASKS};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult,
};
use cw_storage_plus::Bound;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let mut next_task_id = 1u64;

    if let Some(todos) = msg.todos {
        for task_description in todos {
            let id = next_task_id;
            TASKS.save(
                deps.storage,
                id,
                &TodoTask {
                    id,
                    description: task_description,
                    done: false,
                },
            )?;
            next_task_id += 1;
        }
    }
    NEXT_TASK_ID.save(deps.storage, &next_task_id)?;
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Add { task } => {
            // Load next available task ID
            let task_id = NEXT_TASK_ID.load(deps.storage)?;

            // Save the new task
            TASKS.save(
                deps.storage,
                task_id,
                &TodoTask {
                    id: task_id,
                    description: task,
                    done: false,
                },
            )?;

            // Increment the NEXT_TASK_ID
            NEXT_TASK_ID.save(deps.storage, &(task_id + 1))?;

            Ok(Response::new().add_event(event_add_task(&task_id)))
        }
        ExecuteMsg::Toggle { task_id } => {
            let mut task = TASKS.load(deps.storage, task_id)?;
            task.done = !task.done;
            TASKS.save(deps.storage, task_id, &task)?;
            Ok(Response::new()
                .add_event(event_toggle_task(&task_id, &task.done)))
        }
        ExecuteMsg::Delete { task_id } => {
            TASKS.remove(deps.storage, task_id);
            Ok(Response::new().add_event(event_delete_task(&task_id)))
        } // _ => Err(ContractError::NotImplemented),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Todos {} => {
            let tasks: Vec<TodoTask> = TASKS
                .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
                .filter_map(|res| match res {
                    Ok((_, task)) => Some(task),
                    Err(_) => None,
                })
                .collect();
            to_binary(&tasks)
        }
        QueryMsg::TodoTask { id } => {
            let task = TASKS.load(deps.storage, id)?;
            to_binary(&task)
        }
        QueryMsg::CompletedTasks { since_id } => {
            let start_at = since_id.map(Bound::exclusive);
            let tasks: Vec<TodoTask> = TASKS
                .range(
                    deps.storage,
                    start_at,
                    None,
                    cosmwasm_std::Order::Ascending,
                )
                .filter_map(|res| match res {
                    Ok((_, task)) => {
                        if task.done {
                            Some(task)
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                })
                .collect();
            to_binary(&tasks)
        }
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_binary, testing, DepsMut};

    use crate::{
        contract::{execute, instantiate, query},
        events::{event_add_task, event_delete_task, event_toggle_task},
        msgs::{ExecuteMsg, InstantiateMsg, QueryMsg},
        state::TodoTask,
    };

    type TestResult = Result<(), anyhow::Error>;

    #[test]
    fn test_add_task() -> TestResult {
        let mut deps = testing::mock_dependencies();

        // Initialize with empty state
        let init_msg = InstantiateMsg { todos: None };
        instantiate(
            deps.as_mut(),
            testing::mock_env(),
            testing::mock_info("creator", &[]),
            init_msg,
        )?;

        // Add a task
        let msg = ExecuteMsg::Add {
            task: "Learn CosmWasm".to_string(),
        };
        let res = execute(
            deps.as_mut(),
            testing::mock_env(),
            testing::mock_info("anyone", &[]),
            msg,
        )?;

        assert_eq!(res.events, vec![event_add_task(&1u64)]);

        Ok(())
    }

    fn setup_with_tasks(
        deps: DepsMut,
        tasks: Option<Vec<String>>,
    ) -> TestResult {
        let init_msg = InstantiateMsg { todos: tasks };
        instantiate(
            deps,
            testing::mock_env(),
            testing::mock_info("creator", &[]),
            init_msg,
        )?;
        Ok(())
    }

    #[test]
    fn test_toggle_task() -> TestResult {
        let mut deps = testing::mock_dependencies();

        // Initialize with one task
        let init_msg = InstantiateMsg {
            todos: Some(vec!["Learn CosmWasm".to_string()]),
        };
        instantiate(
            deps.as_mut(),
            testing::mock_env(),
            testing::mock_info("creator", &[]),
            init_msg,
        )?;

        // Toggle the task's completion status
        let msg = ExecuteMsg::Toggle { task_id: 1 };
        let res = execute(
            deps.as_mut(),
            testing::mock_env(),
            testing::mock_info("anyone", &[]),
            msg,
        )?;

        assert_eq!(res.events, vec![event_toggle_task(&1u64, &true)]);

        Ok(())
    }

    #[test]
    fn test_delete_task() -> TestResult {
        let mut deps = testing::mock_dependencies();

        // Initialize with one task
        let init_msg = InstantiateMsg {
            todos: Some(vec!["Learn CosmWasm".to_string()]),
        };
        instantiate(
            deps.as_mut(),
            testing::mock_env(),
            testing::mock_info("creator", &[]),
            init_msg,
        )?;

        // Delete the task
        let msg = ExecuteMsg::Delete { task_id: 1 };
        let res = execute(
            deps.as_mut(),
            testing::mock_env(),
            testing::mock_info("anyone", &[]),
            msg,
        )?;

        assert_eq!(res.events, vec![event_delete_task(&1u64)]);

        Ok(())
    }

    #[test]
    fn test_query_todos() -> TestResult {
        let mut deps = testing::mock_dependencies();

        setup_with_tasks(
            deps.as_mut(),
            Some(vec!["Learn Rust".to_string(), "Learn CosmWasm".to_string()]),
        )?;

        // Query all tasks
        let res = query(deps.as_ref(), testing::mock_env(), QueryMsg::Todos {})?;
        let tasks: Vec<TodoTask> = from_binary(&res)?;
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].description, "Learn Rust");
        assert_eq!(tasks[1].description, "Learn CosmWasm");

        Ok(())
    }

    #[test]
    fn test_query_todo_task() -> TestResult {
        let mut deps = testing::mock_dependencies();

        // Initialize with one task
        setup_with_tasks(deps.as_mut(), Some(vec!["Learn Rust".to_string()]))?;

        // Query the specific task by ID
        let res = query(
            deps.as_ref(),
            testing::mock_env(),
            QueryMsg::TodoTask { id: 1 },
        )?;
        let task: TodoTask = from_binary(&res)?;
        assert_eq!(task.description, "Learn Rust");

        Ok(())
    }

    #[test]
    fn test_query_completed_tasks() -> TestResult {
        let mut deps = testing::mock_dependencies();
        setup_with_tasks(
            deps.as_mut(),
            Some(vec!["Learn Rust".to_string(), "Learn CosmWasm".to_string()]),
        )?;

        // Mark the first task as completed
        let toggle_msg = ExecuteMsg::Toggle { task_id: 1 };
        execute(
            deps.as_mut(),
            testing::mock_env(),
            testing::mock_info("anyone", &[]),
            toggle_msg,
        )?;

        // Query completed tasks since ID 0
        let res = query(
            deps.as_ref(),
            testing::mock_env(),
            QueryMsg::CompletedTasks { since_id: Some(0) },
        )?;
        let tasks: Vec<TodoTask> = from_binary(&res)?;
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, "Learn Rust");
        assert!(tasks[0].done);

        Ok(())
    }
}
