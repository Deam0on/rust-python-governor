use crate::runner::execute_python_task;
use crate::logger::log_task_start;
use std::path::Path;
use log::info;

pub fn schedule_tasks(task_path: String) {
    let path = Path::new(&task_path);
    if path.exists() {
        info!("Scheduling task: {}", task_path);
        log_task_start(&task_path);
        execute_python_task(&task_path);
    } else {
        info!("Task file does not exist: {}", task_path);
    }
}
