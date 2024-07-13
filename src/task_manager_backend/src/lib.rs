use candid::CandidType;
use serde::{Serialize, Deserialize};
use std::{cell::RefCell, collections::HashMap};
use std::cell::Cell;

// Task struct
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
struct Task {
    id: u64,
    title: String,
    description: String,
    done: bool,
    is_important: bool,
    created_at: u64,
    updated_at: u64,
}

// Task Manager struct
thread_local! {
    static TASKS: RefCell<HashMap<u64, Task>> = RefCell::default();
    static NEXT_ID: Cell<u64> = Cell::new(0);
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
enum TaskError {
    NotFound,
    InvalidInput,
    DuplicateTask,
}

impl From<TaskError> for String {
    fn from(error: TaskError) -> Self {
        match error {
            TaskError::NotFound => "Task not found".to_string(),
            TaskError::InvalidInput => "Invalid input".to_string(),
            TaskError::DuplicateTask => "Duplicate task".to_string(),
        }
    }
}

#[ic_cdk::update]
fn create_task(title: String, description: String, is_important: Option<bool>) -> Result<u64, String> {
    if title.is_empty() || description.is_empty() {
        return Err(TaskError::InvalidInput.into());
    }

    let id = NEXT_ID.with(|id| {
        let next_id = id.get();
        id.set(next_id + 1);
        next_id
    });

    let timestamp = ic_cdk::api::time();
    let task = Task {
        id,
        title,
        description,
        is_important: is_important.unwrap_or(false),
        done: false,
        created_at: timestamp,
        updated_at: timestamp,
    };

    TASKS.with(|tasks| tasks.borrow_mut().insert(id, task));

    Ok(id)
}

#[ic_cdk::query]
fn get_task(id: u64) -> Result<Task, String> {
    TASKS.with(|tasks| {
        tasks.borrow().get(&id).cloned().ok_or_else(|| TaskError::NotFound.into())
    })
}

#[ic_cdk::query]
fn get_all_tasks() -> Vec<Task> {
    TASKS.with(|tasks| tasks.borrow().values().cloned().collect())
}

#[ic_cdk::update]
fn update_task(id: u64, title: Option<String>, description: Option<String>, done: Option<bool>, is_important: Option<bool>) -> Result<bool, String> {
    TASKS.with(|tasks| {
        if let Some(task) = tasks.borrow_mut().get_mut(&id) {
            if let Some(new_title) = title {
                task.title = new_title;
            }
            if let Some(new_description) = description {
                task.description = new_description;
            }
            if let Some(new_done) = done {
                task.done = new_done;
            }
            if let Some(new_is_important) = is_important {
                task.is_important = new_is_important;
            }
            task.updated_at = ic_cdk::api::time();
            Ok(true)
        } else {
            Err(TaskError::NotFound.into())
        }
    })
}

#[ic_cdk::update]
fn delete_task(id: u64) -> Result<bool, String> {
    TASKS.with(|tasks| tasks.borrow_mut().remove(&id).is_some().then(|| true).ok_or_else(|| TaskError::NotFound.into()))
}

#[ic_cdk::query]
fn search_task_by_status(done: bool) -> Vec<Task> {
    TASKS.with(|tasks| {
        tasks
            .borrow()
            .values()
            .filter(|task| task.done == done)
            .cloned()
            .collect()
    })
}

#[ic_cdk::update]
fn mark_task_as_important(id: u64) -> Result<bool, String> {
    TASKS.with(|tasks| {
        if let Some(task) = tasks.borrow_mut().get_mut(&id) {
            task.is_important = true;
            task.updated_at = ic_cdk::api::time();
            Ok(true)
        } else {
            Err(TaskError::NotFound.into())
        }
    })
}

#[ic_cdk::query]
fn get_important_tasks() -> Vec<Task> {
    TASKS.with(|tasks| {
        tasks
            .borrow()
            .values()
            .filter(|task| task.is_important)
            .cloned()
            .collect()
    })
}

#[ic_cdk::query]
fn get_completed_tasks() -> Vec<Task> {
    TASKS.with(|tasks| {
        tasks
            .borrow()
            .values()
            .filter(|task| task.done)
            .cloned()
            .collect()
    })
}

#[ic_cdk::query]
fn get_incomplete_tasks() -> Vec<Task> {
    TASKS.with(|tasks| {
        tasks
            .borrow()
            .values()
            .filter(|task| !task.done)
            .cloned()
            .collect()
    })
}

#[ic_cdk::query]
fn get_total_number_of_tasks() -> u64 {
    TASKS.with(|tasks| tasks.borrow().len() as u64)
}

#[ic_cdk::query]
fn get_tasks_by_description(description: String) -> Vec<Task> {
    TASKS.with(|tasks| {
        tasks
            .borrow()
            .values()
            .filter(|task| task.description == description)
            .cloned()
            .collect()
    })
}

#[ic_cdk::query]
fn get_tasks_by_importance_status(is_important: bool) -> Vec<Task> {
    TASKS.with(|tasks| {
        tasks
            .borrow()
            .values()
            .filter(|task| task.is_important == is_important)
            .cloned()
            .collect()
    })
}

#[ic_cdk::update]
fn clear_completed_tasks() {
    TASKS.with(|tasks| {
        tasks.borrow_mut().retain(|_, task| !task.done);
    })
}

#[ic_cdk::update]
fn mark_task_as_done(id: u64) -> Result<bool, String> {
    TASKS.with(|tasks| {
        if let Some(task) = tasks.borrow_mut().get_mut(&id) {
            task.done = true;
            task.updated_at = ic_cdk::api::time();
            Ok(true)
        } else {
            Err(TaskError::NotFound.into())
        }
    })
}

#[ic_cdk::update]
fn reset_task_status(id: u64) -> Result<bool, String> {
    TASKS.with(|tasks| {
        if let Some(task) = tasks.borrow_mut().get_mut(&id) {
            task.done = false;
            task.updated_at = ic_cdk::api::time();
            Ok(true)
        } else {
            Err(TaskError::NotFound.into())
        }
    })
}

#[ic_cdk::query]
fn get_tasks_by_title(title: String) -> Vec<Task> {
    TASKS.with(|tasks| {
        tasks
            .borrow()
            .values()
            .filter(|task| task.title == title)
            .cloned()
            .collect()
    })
}

// Additional improvements and functionalities
#[ic_cdk::update]
fn toggle_task_importance(id: u64) -> Result<bool, String> {
    TASKS.with(|tasks| {
        if let Some(task) = tasks.borrow_mut().get_mut(&id) {
            task.is_important = !task.is_important;
            task.updated_at = ic_cdk::api::time();
            Ok(true)
        } else {
            Err(TaskError::NotFound.into())
        }
    })
}

#[ic_cdk::query]
fn get_tasks_created_after(timestamp: u64) -> Vec<Task> {
    TASKS.with(|tasks| {
        tasks
            .borrow()
            .values()
            .filter(|task| task.created_at > timestamp)
            .cloned()
            .collect()
    })
}

#[ic_cdk::query]
fn get_tasks_updated_after(timestamp: u64) -> Vec<Task> {
    TASKS.with(|tasks| {
        tasks
            .borrow()
            .values()
            .filter(|task| task.updated_at > timestamp)
            .cloned()
            .collect()
    })
}

// need this to generate candid
ic_cdk::export_candid!();
