 
use task_manager::TaskManager;

fn main() {
    // Initialize the task manager
    let mut tm = TaskManager::new();

    // Add tasks
    let id1 = tm.add_task("  Buy milk  ".to_string());
    let id2 = tm.add_task("Walk the dog".to_string());

    println!("Tasks added:");
    for id in &[id1, id2] {
        let task = tm.get_task(*id).unwrap();
        println!("  {}: {:?}", id, task);
    }

    // Complete a task
    tm.complete_task(id1).unwrap();

    // Show updated tasks
    println!("\nAfter completing task {}:", id1);
    for id in &[id1, id2] {
        let task = tm.get_task(*id).unwrap();
        println!("  {}: {:?}", id, task);
    }
        // Delete a task
    println!("\nDeleting task {}...", id2);
    tm.delete_task(id2).unwrap();

    // Show tasks after deletion
    println!("\nTasks after deletion:");
    for (id, task) in tm.all_tasks() {
        println!("  {}: {:?}", id, task);
    }

    // Try to complete a non-existent task
    match tm.complete_task(999) {
        Ok(_) => println!("Unexpectedly succeeded on invalid task"),
        Err(e) => println!("Failed to complete non-existent task (expected): {:?}", e),
    }
}
