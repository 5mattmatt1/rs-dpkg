#!/usr/bin/python3

import subprocess
import os
import json
from random import choice
from glob import glob

NUM_PACKAGES = 100

def build_cache():
    if os.path.exists("cache.json"):
        print("Cache already created. Halting")
        return

    output = subprocess.check_output(["apt-cache", "search", "."]).decode('utf-8')
    packages = [pkg.split(" - ")[0] for pkg in output.split('\n')]
    if not os.path.exists("cache"):
        os.mkdir("cache")
    to_download = []
    for _ in range(NUM_PACKAGES):    
        pkg = None
        while pkg is None or pkg in to_download:
            pkg = choice(packages)
        to_download.append(pkg)
        # print(pkg)
    print("Number of packages to download: {}".format(len(to_download)))

    # BUILD CACHE
    cache = {}
    for pkg in to_download:
        subprocess.run(["apt-get", "download", pkg])
        glob_deb = glob("*.deb")
        for deb in glob_deb:
            if deb not in cache.values():
                cache[pkg] = deb
                break
    
    with open("cache.json", "w") as f:
        json.dump(cache, f, sort_keys=True, indent=4)

def get_cache():
    with open("cache.json", "r") as f:
        cache = json.load(f)
    return cache

def install_cache():
    for pkg_name, pkg_deb in get_cache().items():
        pkg_deb_pth = os.path.join("cache", pkg_deb)
        subprocess.run(["sudo", "dpkg", "-i", pkg_deb_pth])

def remove_cache():
    if not os.path.exists("cache.json"):
        print("Can't remove cache until built. Halting.")
        return

def purge_cache():
    if not os.path.exists("cache.json"):
        print("Can't purge cache until built. Halting.")
        return

def get_cache_status(postfix):
    for pkg_name, pkg_deb in get_cache().items():
        pkg_stat = subprocess.check_output(["../run.sh", "debug", pkg_name])
        pkg_stat_pth = os.path.join("status", "{}.{}".format(pkg_name, postfix))
        with open(pkg_stat_pth, "w") as f:
            f.write(pkg_stat)

def main():
    # install_cache()
    get_cache_status("install")

if __name__ == "__main__":
    main()
