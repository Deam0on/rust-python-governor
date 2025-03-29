# Rust + Python Task Governor

This project provides a hybrid Rust + Python task management system.

## Features
- Rust binary watches a directory for new `.yaml` task files
- Python scripts are triggered via a unified interface
- Logs and output are stored locally
- Modular, extensible, cross-platform

## Requirements
- Rust 1.65+
- Python 3.7+

## Quick Start
```bash
# Install Rust dependencies
cargo build

# Install Python dependencies
pip install -r requirements.txt

# Run the governor
cargo run

# Add a task YAML under ./tasks/
cp py_runner/example_task/task.yaml tasks/example.yaml
```

## Python Task Spec
Each task folder must contain:
- `task.yaml`: Contains `script` and `params`
- A Python script implementing `run(params: dict)`

## Example `task.yaml`
```yaml
script: task_script.py
params:
  message: "Hello World"
  repeat: 2
```

## License
MIT
