def build(function):
    def f(*args, **kwargs):
        import subprocess
        subprocess.run(["python3","../Rust_Cargo/script.py"],cwd="../Rust_Cargo")
        result = function(*args, **kwargs)
        return result
    return f
