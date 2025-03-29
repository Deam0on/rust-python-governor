use std::process::Command;
use std::path::Path;
use log::{error, info};

pub fn execute_python_task(task_yaml_path: &str) {
    let path = Path::new(task_yaml_path);
    let task_dir = path.parent().unwrap_or_else(|| Path::new("."));

    let output = Command::new("python")
        .arg("py_runner/task_interface.py")
        .arg(task_yaml_path)
        .current_dir(task_dir)
        .output();

    match output {
        Ok(output) => {
            info!("Python stdout: {}", String::from_utf8_lossy(&output.stdout));
            info!("Python stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
        Err(e) => {
            error!("Failed to execute Python task: {}", e);
        }
    }
}
