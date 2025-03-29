def run(params):
    message = params.get("message", "No message provided")
    repeat = int(params.get("repeat", 1))
    for i in range(repeat):
        print(f"[{i+1}] {message}")
    return {"status": "success", "message": message, "repeat": repeat}
