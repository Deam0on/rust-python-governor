use chrono::Local;
use env_logger::Builder;
use log::info;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn init_logger() {
    Builder::from_default_env()
        .format_timestamp_secs()
        .init();
}

pub fn log_task_start(task_path: &str) {
    let log_dir = Path::new("logs");
    if let Err(e) = create_dir_all(log_dir) {
        eprintln!("Failed to create log directory: {}", e);
        return;
    }

    let log_file_path = log_dir.join("task_log.txt");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
        .expect("Unable to open log file");

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_entry = format!("[{}] Task scheduled: {}
", timestamp, task_path);

    if let Err(e) = file.write_all(log_entry.as_bytes()) {
        eprintln!("Failed to write log: {}", e);
    }

    info!("{}", log_entry.trim());
}
