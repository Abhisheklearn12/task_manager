use task_manager::TaskManager;

#[test]
fn test_add_and_complete_task() {
    let mut tm = TaskManager::new();
    let id = tm.add_task("  Buy milk  ".to_string());
    
    tm.complete_task(id).unwrap();
    let task = tm.get_task(id).unwrap();
    
    assert_eq!(task.title, "Buy milk"); // ✅ now passes
    assert!(task.done);
}

#[test]
fn test_complete_nonexistent_task() {
    let mut tm = TaskManager::new();
    let result = tm.complete_task(999);
    assert!(matches!(result, Err(_)));
}
