// src/watcher.rs

use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;
use log::info;

pub fn watch_directory<F>(path: &Path, callback: F)
where
    F: Fn(String) + Send + 'static + Copy,
{
    let (tx, rx) = channel();

    // RecommendedWatcher is the unified async watcher interface in notify v6
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Config::default()).expect("Failed to initialize watcher");
    watcher
        .watch(path, RecursiveMode::Recursive)
        .expect("Failed to watch path");

    info!("Watching directory: {}", path.display());

    thread::spawn(move || {
        for res in rx {
            match res {
                Ok(Event { kind: EventKind::Create(_) | EventKind::Modify(_), paths, .. }) => {
                    for path in paths {
                        if let Some(ext) = path.extension() {
                            if ext == "yaml" || ext == "json" {
                                info!("Detected task file: {}", path.display());
                                callback(path.to_string_lossy().into_owned());
                            }
                        }
                    }
                }
                Ok(_) => {}
                Err(e) => eprintln!("watch error: {e}"),
            }
        }
    });
}
