use std::path::Path;

mod task;
use task::TaskManager;

fn main() {
    let mut tm = TaskManager::new();
    tm.read_from_csv(Path::new("tasks.csv"), true);

    println!("\nAll tasks:");
    tm.print_tasks();

    println!("\nSorted by planned date:");
    tm.sort_by_planned_from();
    tm.print_tasks();

    println!("\nSorted by planned duration:");
    tm.sort_by_planned_duration();
    tm.print_tasks();

    println!("\nSorted by priority:");
    tm.sort_by_priority();
    tm.print_tasks();
}
