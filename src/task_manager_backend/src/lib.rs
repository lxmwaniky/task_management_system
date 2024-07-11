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
}

// Task Manager struct
thread_local! {
    static TASKS: RefCell<HashMap<u64, Task>> = RefCell::default();
    static NEXT_ID: Cell<u64> = Cell::new(0);
}

#[ic_cdk::update]
// Create a new task
fn create_task(title: String, description: String, is_important: Option<bool>) -> Result<u64, String> {
    // Validate input payload
    if title.is_empty() || description.is_empty() {
        return Err("Title and description must be provided and non-empty".to_string());
    }

    // Increment the ID counter and create a new task
    let id = NEXT_ID.with(|id| {
        let next_id = id.get();
        id.set(next_id + 1);
        next_id
    });

    let task = Task {
        id,
        title,
        description,
        is_important: is_important.unwrap_or(false),
        done: false,
    };

    // Insert the new task into storage
    TASKS.with(|tasks| tasks.borrow_mut().insert(id, task));

    Ok(id)
}

#[ic_cdk::query]
// Get a task by ID
fn get_task(id: u64) -> Option<Task> {
    TASKS.with(|tasks| tasks.borrow().get(&id).cloned())
}

#[ic_cdk::query]
// Get all tasks
fn get_all_tasks() -> Vec<Task> {
    TASKS.with(|tasks| tasks.borrow().values().cloned().collect())
}

#[ic_cdk::update]
// Update task details
fn update_task(id: u64, title: Option<String>, description: Option<String>, done: Option<bool>, is_important: Option<bool>) -> Result<bool, String> {
    // Validate input payload
    if title.is_empty() || description.is_empty() {
        return Err("Title and description must be provided and non-empty".to_string());
    }

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
            Ok(true)
        } else {
            Err("Task not found".to_string())
        }
    })
}

#[ic_cdk::update]
// Delete a task
fn delete_task(id: u64) -> bool {
    TASKS.with(|tasks| tasks.borrow_mut().remove(&id).is_some())
}

#[ic_cdk::query]
// Search tasks by completion status
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
// Mark task as important
fn mark_task_as_important(id: u64) -> Result<bool, String> {
    TASKS.with(|tasks| {
        if let Some(task) = tasks.borrow_mut().get_mut(&id) {
            task.is_important = true;
            Ok(true)
        } else {
            Err("Task not found".to_string())
        }
    })
}

#[ic_cdk::query]
// Get all important tasks
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
// Get all completed tasks
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
// Get all incomplete tasks
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
// Get total number of tasks
fn get_total_number_of_tasks() -> u64 {
    TASKS.with(|tasks| tasks.borrow().len() as u64)
}

#[ic_cdk::query]
// Get tasks by description
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
// Get tasks by importance status
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
// Clear all completed tasks
fn clear_completed_tasks() {
    TASKS.with(|tasks| {
        tasks.borrow_mut().retain(|_, task| !task.done);
    })
}

#[ic_cdk::update]
// Mark task as done
fn mark_task_as_done(id: u64) -> Result<bool, String> {
    TASKS.with(|tasks| {
        if let Some(task) = tasks.borrow_mut().get_mut(&id) {
            task.done = true;
            Ok(true)
        } else {
            Err("Task not found".to_string())
        }
    })
}

#[ic_cdk::update]
// Reset task status to not done
fn reset_task_status(id: u64) -> Result<bool, String> {
    TASKS.with(|tasks| {
        if let Some(task) = tasks.borrow_mut().get_mut(&id) {
            task.done = false;
            Ok(true)
        } else {
            Err("Task not found".to_string())
        }
    })
}

#[ic_cdk::query]
// Get tasks by title
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
