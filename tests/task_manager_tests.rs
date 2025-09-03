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
