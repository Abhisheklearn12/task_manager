 
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Task {
    pub title: String,
    pub done: bool,
}

#[derive(Debug)]
pub struct TaskManager {
    tasks: HashMap<u32, Task>,
    next_id: u32,
}

#[derive(Debug)]
pub enum TaskError {
    NotFound,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, title: String) -> u32 {
        let id = self.next_id;
        self.tasks.insert(id, Task { title, done: false });
        self.next_id += 1;
        id
    }

    pub fn complete_task(&mut self, id: u32) -> Result<(), TaskError> {
        match self.tasks.get_mut(&id) {
            Some(task) => {
                task.title = task.title.trim().to_string(); // fix
                task.done = true;
                Ok(())
            }
            None => Err(TaskError::NotFound),
        }
    }

    pub fn get_task(&self, id: u32) -> Result<&Task, TaskError> {
        self.tasks.get(&id).ok_or(TaskError::NotFound)
    }

pub fn delete_task(&mut self, id: u32) -> Result<(), TaskError> {
    match self.tasks.remove(&id) {
        Some(_) => Ok(()),
        None => Err(TaskError::NotFound),
    }
}
    pub fn all_tasks(&self) -> impl Iterator<Item = (&u32, &Task)> {
        self.tasks.iter()
    }


// --- New Features ---

/// Edit the title of a task
pub fn edit_task(&mut self, id: u32, new_title: String) -> Result<(), TaskError> {
    match self.tasks.get_mut(&id) {
        Some(task) => {
            task.title = new_title.trim().to_string();
            Ok(())
        }
        None => Err(TaskError::NotFound),
    }
}

/// Return only completed tasks
pub fn completed_tasks(&self) -> Vec<(&u32, &Task)> {
    self.tasks.iter().filter(|(_, t)| t.done).collect()
}

/// Return only pending tasks
pub fn pending_tasks(&self) -> Vec<(&u32, &Task)> {
    self.tasks.iter().filter(|(_, t)| !t.done).collect()
}

/// Get counts of tasks
pub fn task_counts(&self) -> (usize, usize, usize) {
    let total = self.tasks.len();
    let completed = self.tasks.values().filter(|t| t.done).count();
    let pending = total - completed;
    (total, completed, pending)
}

/// Clear all completed tasks
pub fn clear_completed(&mut self) {
    self.tasks.retain(|_, t| !t.done);
}

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_complete_task() {
        let mut tm = TaskManager::new();
        let id = tm.add_task("  Buy milk  ".to_string());

        tm.complete_task(id).unwrap();
        let task = tm.get_task(id).unwrap();

        assert_eq!(task.title, "Buy milk");
        assert!(task.done);
    }

    #[test]
    fn test_complete_nonexistent_task() {
        let mut tm = TaskManager::new();
        let result = tm.complete_task(999);
        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn test_delete_existing_task() {
        let mut tm = TaskManager::new();
        let id = tm.add_task("Buy milk".to_string());

        // delete the task
        let result = tm.delete_task(id);
        assert!(result.is_ok());

        // task should no longer exist
        let task = tm.get_task(id);
        assert!(task.is_err());
    }

    #[test]
    fn test_delete_nonexistent_task() {
        let mut tm = TaskManager::new();

        // deleting a non-existent task should return Err
        let result = tm.delete_task(999);
        assert!(matches!(result, Err(TaskError::NotFound)));
    }
#[test]
fn test_edit_task() {
    let mut tm = TaskManager::new();
    let id = tm.add_task("Old title".to_string());

    tm.edit_task(id, "New title".to_string()).unwrap();
    let task = tm.get_task(id).unwrap();
    assert_eq!(task.title, "New title");
}

#[test]
fn test_completed_and_pending_tasks() {
    let mut tm = TaskManager::new();
    let id1 = tm.add_task("Task 1".to_string());
    let _id2 = tm.add_task("Task 2".to_string());

    tm.complete_task(id1).unwrap();

    let completed = tm.completed_tasks();
    let pending = tm.pending_tasks();

    assert_eq!(completed.len(), 1);
    assert_eq!(pending.len(), 1);
    assert_eq!(completed[0].1.title, "Task 1");
    assert_eq!(pending[0].1.title, "Task 2");
}

#[test]
fn test_task_counts() {
    let mut tm = TaskManager::new();
    let id1 = tm.add_task("Task 1".to_string());
    tm.add_task("Task 2".to_string());

    tm.complete_task(id1).unwrap();

    let (total, completed, pending) = tm.task_counts();
    assert_eq!(total, 2);
    assert_eq!(completed, 1);
    assert_eq!(pending, 1);
}

#[test]
fn test_clear_completed() {
    let mut tm = TaskManager::new();
    let id1 = tm.add_task("Task 1".to_string());
    tm.add_task("Task 2".to_string());

    tm.complete_task(id1).unwrap();
    tm.clear_completed();

    let (total, completed, pending) = tm.task_counts();
    assert_eq!(total, 1);
    assert_eq!(completed, 0);
    assert_eq!(pending, 1);
}


}
