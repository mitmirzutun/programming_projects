import subprocess
def main():
    subprocess.run(["cargo", "build"], cwd="milans_pyo3_library")
    subprocess.run(["cp"])
    subprocess.run(["cargo", "fmt"])
    subprocess.run(["cargo", "test"],check=True)
    subprocess.run(["cargo", "run"])
if __name__=="__main__":
    main()