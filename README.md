# Task Management System

## Rust-based Task Management System

This Rust-based Canister provides a task management system that allows you to perform basic operations on tasks. It utilizes the Internet Computer Candid interface for communication and ic_cdk for initialization.

### Initialization

To get started, clone the repository and navigate to the project directory:

```bash
git clone https://github.com/lxmwaniky/task_management_system.git
cd task_management_system
```

### Prerequisites

Before running the task management system, make sure you have Rust and the Internet Computer Canister Development Kit (ic_cdk) installed. You can find the Rust infrastructure and installation instructions [here](https://internetcomputer.org/docs/current/developer-docs/backend/rust/dev-env).

### Run Internet Computer

Start the Internet Computer Replica:

```bash
dfx start --clean --background
```

Deploy the Task Manager Canister:

```bash
dfx deploy task_manager_backend
```

This will deploy the Task Manager Canister, and you will receive output containing information about the deployed canister and its identifiers.

## Functions

The task manager allows you to perform the following actions:

### Create_task

**Description**: Creates a new task with the given title and description.

**Parameters**:

- `title` (String): The title of the task.
- `description` (String): The description of the task.

**Returns**:

- `id` (u64): The unique identifier assigned to the newly created task.

**Usage**:

```rust
let task_id = create_task("Task Title".to_string(), "Task Description".to_string());
```

### get_task

**Description**: Retrieves a task by its unique identifier.

**Parameters**:

- `id` (u64): The unique identifier of the task.

**Returns**:

- `Option<Task>`: The task if found, otherwise None.

**Usage**:

```rust
if let Some(task) = get_task(123) {
    // Handle the task
} else {
    // Task not found
}
```

### get_all_tasks

**Description**: Retrieves all tasks currently stored.

**Returns**:

- `Vec<Task>`: A vector containing all tasks.

**Usage**:

```rust
let all_tasks = get_all_tasks();
for task in all_tasks {
    // Process each task
}
```

### update_task_status

**Description**: Updates the status (title, description, and completion) of a task.

**Parameters**:

- `id` (u64): The unique identifier of the task.
- `title` (String): The new title of the task.
- `description` (String): The new description of the task.
- `done` (bool): The new completion status of the task.

**Returns**:

- `bool`: true if the update was successful, false otherwise.

**Usage**:

```rust
let updated = update_task_status(123, "New Title".to_string(), "New Description".to_string(), true);
```

### update_task

**Description**: Updates either the title or description of a task.

**Parameters**:

- `id` (u64): The unique identifier of the task.
- `title` (Option<String>): The new title of the task (if provided).
- `description` (Option<String>): The new description of the task (if provided).

**Returns**:

- `bool`: true if the update was successful, false otherwise.

**Usage**:

```rust
let updated = update_task(123, Some("New Title".to_string()), None);
```

### delete_task

**Description**: Deletes a task by its unique identifier.

**Parameters**:

- `id` (u64): The unique identifier of the task.

**Returns**:

- `bool`: true if the deletion was successful, false otherwise.

**Usage**:

```rust
let deleted = delete_task(123);
```

### search_task_by_status

**Description**: Searches for tasks based on their completion status.

**Parameters**:

- `done` (bool): The completion status to search for.

**Returns**:

- `Vec<Task>`: A vector containing tasks with the specified completion status.

**Usage**:

```rust
let completed_tasks = search_task_by_status(true);
for task in completed_tasks {
    // Process each completed task
}
```