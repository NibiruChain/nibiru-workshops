use cosmwasm_std::Event;
// use crate::state::TodoTask;
//
pub fn event_add_task(id: &u64) -> Event {
    Event::new("add_task").add_attribute("id", id.to_string())
}

pub fn event_toggle_task(id: &u64, new_done: &bool) -> Event {
    Event::new("toggle_task")
        .add_attribute("id", id.to_string())
        .add_attribute("new_done", new_done.to_string())
}

pub fn event_delete_task(id: &u64) -> Event {
    Event::new("delete_task").add_attribute("id", id.to_string())
}
