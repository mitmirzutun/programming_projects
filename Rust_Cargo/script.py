import itertools
import os
import subprocess
import sys
import tomllib

def get_all_workspaces(crate_dir: str | bytes) -> list[str]:
    workspaces = list()
    workspaces.append(crate_dir)
    with open(os.path.join(crate_dir, "Cargo.toml"), "rb") as toml_file:
        toml = tomllib.load(toml_file)
    if "workspace" in toml and "members" in toml["workspace"]:
        for workspace in toml["workspace"]["members"]:
            workspaces += get_all_workspaces(os.path.join(crate_dir, workspace))
    return workspaces


def pipe(cmd: list[str], path: str | bytes) -> None:
    result = subprocess.run(cmd, capture_output=True)
    with open(path, "wb") as file:
        file.write(result.stdout)


def expand(crate_dir: str | bytes) -> None:
    manifest_path = os.path.join(crate_dir, "Cargo.toml")
    with open(manifest_path, "rb") as toml_file:
        toml = tomllib.load(toml_file)
    name = toml["package"]["name"]
    features = []
    binaries = []
    if "features" in toml:
        tmp = dict(toml["features"])
        del tmp["default"]
        tmp = tmp.keys()
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


def build():
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
            result = subprocess.run(["cargo", "test", "--manifest-path", os.path.join(workspace, "Cargo.toml")],
                                    capture_output=True)
            if result.returncode != 0:
                subprocess.run(["cargo", "test", "--manifest-path", os.path.join(workspace, "Cargo.toml")])
                break
            result = subprocess.run(["cargo", "build", "--manifest-path", os.path.join(workspace, "Cargo.toml")],
                                    capture_output=True)
            if result.returncode != 0:
                subprocess.run(["cargo", "build", "--manifest-path", os.path.join(workspace, "Cargo.toml")])
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
