use task_manager::TaskManager;

#[test]
fn test_add_and_complete_task() {
    let mut tm = TaskManager::new();
    let id = tm.add_task("  Buy milk  ".to_string());
    
    tm.complete_task(id).unwrap();
    let task = tm.get_task(id).unwrap();
    
    assert_eq!(task.title, "Buy milk"); // now passes clean
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
    let id = tm.add_task("Buy eggs".to_string());

    assert!(tm.delete_task(id).is_ok());
    assert!(tm.get_task(id).is_err());
}

#[test]
fn test_delete_nonexistent_task() {
    let mut tm = TaskManager::new();
    let result = tm.delete_task(12345);
    assert!(matches!(result, Err(_)));
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

#[test]
fn test_all_tasks_iterator() {
    let mut tm = TaskManager::new();
    let id1 = tm.add_task("Task A".to_string());
    let id2 = tm.add_task("Task B".to_string());

    let mut ids = tm.all_tasks().map(|(id, _)| *id).collect::<Vec<_>>();
    ids.sort();
    assert_eq!(ids, vec![id1, id2]);
}
