import itertools
import os
import subprocess
import sys
import tomllib
CARGO_TOML = "Cargo.toml"

def get_all_workspaces(crate_dir: str) -> list[str]:
    workspaces = list()
    workspaces.append(crate_dir)
    with open(os.path.join(crate_dir, CARGO_TOML), "rb") as toml_file:
        toml = tomllib.load(toml_file)
    if "workspace" in toml and "members" in toml["workspace"]:
        for workspace in toml["workspace"]["members"]:
            workspaces += get_all_workspaces(os.path.join(crate_dir, workspace))
    return workspaces


def pipe(cmd: list[str], path: str | bytes) -> None:
    result = subprocess.run(cmd, capture_output=True)
    with open(path, "wb") as file:
        file.write(result.stdout)


def expand(crate_dir: str) -> None:
    manifest_path = os.path.join(crate_dir, CARGO_TOML)
    with open(manifest_path, "rb") as toml_file:
        toml = tomllib.load(toml_file)
    name = toml["package"]["name"]
    features: list[str] = []
    binaries = []
    if "features" in toml:
        tmp_dict = dict(toml["features"])
        del tmp_dict["default"]
        tmp = tmp_dict.keys()
        for length in range( len(tmp) + 1):
            features += map(",".join, itertools.combinations(tmp, length))
    if "bin" in toml:
        for binary in toml["bin"]:
            binaries.append(binary["name"])
    if features != [] and binaries != []:
        for feature in features:
            pipe(["cargo", "expand", "--lib", "--no-default-features", f"--features={feature}", "--manifest-path",
                  manifest_path], os.path.join("expanded", name + "_features_" + feature + ".rs"))
            for binary in binaries:
                pipe(["cargo", "expand", "--binary", binary, "--no-default-features", f"--features={feature}",
                      "--manifest-path", manifest_path], os.path.join("expanded",
                                                                      name + "_features_" + feature + "_bin_" + binary + ".rs"))
    elif len(binaries) != 0:
        for binary in binaries:
            pipe(["cargo", "expand", "--bin", binary, "--mainfest-path", manifest_path],
                 os.path.join("expanded", name + "_bin_" + binary + ".rs"))
    elif len(features) != 0:
        for feature in features:
            pipe(["cargo", "expand", "--lib", "--no-default-features", f"--features={feature}", "--manifest-path",
                  manifest_path], os.path.join("expanded", name + "_features_" + feature + ".rs"))
    pipe(["cargo", "expand", "--lib", "--manifest-path", manifest_path], os.path.join("expanded", name+".rs"))


def get_all_files(root:str):
    import os
    if os.path.isfile(root):
        yield root
        return
    if os.path.isdir(root):
        result=subprocess.run(["ls",root],capture_output=True).stdout.decode("utf8").split("\n")
        for file in result:
            if file=="":
                continue
            yield from get_all_files(os.path.join(root,file))


def build(on_change:bool=True):
    if on_change:
        import os,time
        src_latest=0
        for file in get_all_files("src"):
            src_latest=max(src_latest,os.path.getmtime(file))
        for file in get_all_files(os.path.join("milans_rust_core","src")):
            src_latest=max(src_latest,os.path.getmtime(file))
        for file in get_all_files(os.path.join("milans_pyo3_library","src")):
            src_latest=max(src_latest,os.path.getmtime(file))
        bin_latest=0
        for file in get_all_files("target"):
            bin_latest=max(bin_latest,os.path.getmtime(file))
        exp_latest=0
        for file in get_all_files("expanded"):
            exp_latest=max(exp_latest,os.path.getmtime(file))
        if bin_latest>src_latest and exp_latest>src_latest:
            return
    c = ""
    while c != "c":
        subprocess.run(["cargo", "fmt"])
        return_code = subprocess.call(["cargo", "clippy"])
        if return_code != 0:
            c = input("continue?")
            continue
        for workspace in get_all_workspaces("."):
            print(f"Expanding and testing workspace {workspace}")
            expand(workspace)
            result = subprocess.run(["cargo", "test", "--manifest-path", os.path.join(workspace, CARGO_TOML)],
                                    capture_output=True)
            if result.returncode != 0:
                subprocess.run(["cargo", "test", "--manifest-path", os.path.join(workspace, CARGO_TOML)])
                break
            result = subprocess.run(["cargo", "build", "--manifest-path", os.path.join(workspace, CARGO_TOML)],
                                    capture_output=True)
            if result.returncode != 0:
                subprocess.run(["cargo", "build", "--manifest-path", os.path.join(workspace, CARGO_TOML)])
                break
        c = input("continue?")
        

def main():
    if len(sys.argv)>1 and sys.argv=="test":
        build()
    else:
        build()
        subprocess.run(["cp", "target/debug/libmilans_pyo3_library.so",
                        "../Python/.venv/lib/python3.11/site-packages/milans_pyo3_library.so"])


if __name__ == "__main__":
    main()
