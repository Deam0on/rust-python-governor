mod watcher;
mod scheduler;
mod logger;
mod runner;

use std::path::PathBuf;
use crate::watcher::watch_directory;
use crate::scheduler::schedule_tasks;
use crate::logger::init_logger;

fn main() {
    init_logger();
    let watch_path = PathBuf::from("./tasks");
    watch_directory(&watch_path, schedule_tasks);
}
