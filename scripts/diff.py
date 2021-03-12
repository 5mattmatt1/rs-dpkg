#!/usr/bin/python3
import subprocess
import os

AIFPATH = "install_status.txt"
BIFPATH = "remove_status.txt"

def diff(aifpath, bifpath):
    with open(aifpath, "r") as af:
        aibuffer = af.read()
        aobuffer = '\n'.join(sorted(aibuffer.split('\n')))
    
    with open(bifpath, "r") as bf:
        bibuffer = bf.read()
        bobuffer = '\n'.join(sorted(bibuffer.split('\n')))

    with open("a.temp", "w") as af:
        af.write(aobuffer)
    
    with open("b.temp", "w") as bf:
        bf.write(bobuffer)
    
    subprocess.run(["diff", "a.temp", "b.temp"])
    os.remove("a.temp")
    os.remove("b.temp")

def main():
    diff(AIFPATH, BIFPATH)

if __name__ == "__main__":
    main()