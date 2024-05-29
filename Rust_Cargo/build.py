import subprocess,os,tomllib
def get_all_files(path):
    if os.path.isfile(path):
        yield path
    if os.path.isdir(path):
        for folder in os.walk(path):
            yield from map(lambda x: os.path.join(folder[0],x),folder[2])
def main():
    __path__=os.path.dirname(__file__)
    with open(os.path.join(__path__,"Cargo.toml"),"rb") as tomlfile:
        workspaces=tomllib.load(tomlfile)["workspace"]["members"]
    workspaces=list(map(lambda x: os.path.join(__path__,x,"src"),workspaces))
    workspaces.append(os.path.join(__path__,"src"))
    source_files=[__file__]
    for workspace in workspaces:
        source_files += list(get_all_files(workspace))
    target_files=get_all_files(os.path.join(__path__,"target"))
    if os.path.getmtime(max(source_files,key=os.path.getmtime))<os.path.getmtime(max(target_files,key=os.path.getmtime)):
        return
    subprocess.run(["cargo", "build"], cwd=os.path.join(__path__,"milans_pyo3_library"))
    subprocess.run(["cp","target/debug/libmilans_pyo3_library.so","../Python/.venv/lib/python3.11/site-packages/milans_pyo3_library.so"],cwd=__path__)
    subprocess.run(["cp","target/debug/libmilans_pyo3_library.so","../Python/milans_pyo3_library.so"],cwd=__path__)
    result=subprocess.run(["cargo","expand"], cwd=os.path.join(__path__,"milans_rust_core"), capture_output=True)
    with open(os.path.join(__path__,"milans_rust_core","expanded.rs"),"wb") as expanded:
        expanded.write(result.stdout)
    subprocess.run(["cargo", "fmt"], cwd=__path__)
    subprocess.run(["cargo", "clippy"], check=True, cwd=__path__)
    subprocess.run(["cargo", "test"], check=True, cwd=__path__)
    subprocess.run(["cargo", "test"], check=True, cwd=os.path.join(__path__,"milans_rust_core"))
    subprocess.run(["cargo", "test"], check=True, cwd=os.path.join(__path__,"milans_rust_proc_macros"))
    subprocess.run(["cargo", "test"], check=True, cwd=os.path.join(__path__,"milans_pyo3_library"))
    #subprocess.run(["cargo", "run"], cwd=__path__)
if __name__=="__main__":
    main()
