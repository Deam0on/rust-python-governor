import sys
import yaml
import importlib.util
from pathlib import Path

def load_task_module(script_path):
    spec = importlib.util.spec_from_file_location("task_module", script_path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

def run_task(task_yaml_path):
    with open(task_yaml_path, 'r') as f:
        config = yaml.safe_load(f)

    script = config.get("script")
    if not script:
        raise ValueError("Missing 'script' field in task.yaml")

    task_path = Path(task_yaml_path).parent / script
    module = load_task_module(task_path)

    if not hasattr(module, "run"):
        raise AttributeError("The task script must define a 'run()' function")

    print(f"Running task: {task_yaml_path}")
    result = module.run(config.get("params", {}))
    print("Task result:", result)

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python task_interface.py path/to/task.yaml")
        sys.exit(1)

    run_task(sys.argv[1])
