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
                task.done = true;
                Ok(())
            }
            None => Err(TaskError::NotFound),
        }
    }

    pub fn get_task(&self, id: u32) -> Result<&Task, TaskError> {
        self.tasks.get(&id).ok_or(TaskError::NotFound)
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
}
