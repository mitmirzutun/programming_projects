import subprocess
def main():
    import os
    __path__=os.path.dirname(__file__)
    print(__path__)
    subprocess.run(["cargo", "build"], cwd=os.path.join(__path__,"milans_pyo3_library"))
    result=subprocess.run(["cargo","expand"], cwd=os.path.join(__path__,"milans_rust_core"), capture_output=True)
    with open(os.path.join(__path__,"milans_rust_core","expanded.rs"),"wb") as expanded:
        expanded.write(result.stdout)
    subprocess.run(["cargo", "fmt"], cwd=__path__)
    subprocess.run(["cargo", "clippy"], check=True, cwd=__path__)
    subprocess.run(["cargo", "test"], check=True, cwd=__path__)
    subprocess.run(["cargo", "test"], check=True, cwd=os.path.join(__path__,"milans_rust_core"))
    subprocess.run(["cargo", "test"], check=True, cwd=os.path.join(__path__,"milans_rust_proc_macros"))
    subprocess.run(["cargo", "test"], check=True, cwd=os.path.join(__path__,"milans_pyo3_library"))
    subprocess.run(["cargo", "run"], cwd=__path__)
if __name__=="__main__":
    main()