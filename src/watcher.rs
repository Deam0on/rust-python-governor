use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::thread;
use log::info;

pub fn watch_directory<F>(path: &Path, callback: F)
where
    F: Fn(String) + Send + 'static + Copy,
{
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(2)).expect("Failed to create watcher");
    watcher
        .watch(path, RecursiveMode::Recursive)
        .expect("Failed to watch path");

    info!("Watching directory: {}", path.display());

    thread::spawn(move || {
        for event in rx {
            match event {
                DebouncedEvent::Create(path) | DebouncedEvent::Write(path) => {
                    if let Some(ext) = path.extension() {
                        if ext == "yaml" || ext == "json" {
                            info!("Detected new task file: {}", path.display());
                            callback(path.to_string_lossy().into_owned());
                        }
                    }
                }
                _ => {}
            }
        }
    });
}
