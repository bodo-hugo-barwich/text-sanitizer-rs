#!/usr/bin/python3

import sys
import os
import os.path
import toml
import json

# ==============================================================================
# Auxiliary Functions


def list_cargo_files(directory):
    dir = os.scandir(directory)
    cargo_files = {'success': True, 'files': []}
    sub_dirs = []

    for entry in dir:
        if entry.is_file():
            if os.path.splitext(entry.name)[1] == '.toml':
                cargo_files['files'].append(directory + entry.name)

        elif entry.is_dir():
            sub_dirs.append(entry.name)

    for sub_dir in sub_dirs:
        sub_files = list_cargo_files(directory + sub_dir + '/')

        if not sub_files['success']:
            cargo_files['success'] = False

        if len(sub_files['files']) > 0:
            for file in sub_files['files']:
                cargo_files['files'].append(file)

    return cargo_files


def parse_cargo_files(cargo_files):
    packages_res = {'success': True, 'packages': {}}

    for file in cargo_files:
        package = {}
        directory_name = parse_parent_directory_name(file)
        package_name = directory_name

        if module_debug:
            print(
                "parent dir name:'{}'".format(directory_name))

        try:
            with open(file, 'r') as f:
                package = toml.load(f)
                f.close()

            if module_debug:
                print("script '{}' - Cargo File '{}': File read.".format(
                    module_file, file))

            if module_debug:
                print(
                    "script '{}' - Cargo Package:\n{}".format(module_file, str(package)))

            if 'package' in package:
                if 'name' in package:
                    package_name = package.name
            elif 'workspace' in package:
                package_name = package_name + '_workspace'

        except Exception as e:
            if not module_quiet:
                print(
                    "script '{}' - Cargo File '{}': Read File failed!".format(
                        module_file, file), file=sys.stderr)
                print("script '{}' - Cargo File Exception Message: {}".format(
                    module_file, str(e)), file=sys.stderr)

            packages.result = 0

        if 'package_name' not in packages_res['packages']:
            packages_res['packages'][package_name] = package
        else:
            packages_res['packages'][directory_name] = package

    return packages_res


def parse_parent_directory_name(filepath):
    parent_dir = ''
    parent_name = ''
    file_name = ''
    slash_pos = filepath.rfind('/', 0)

    if slash_pos != -1:
        parent_dir = filepath[0: slash_pos + 1]
        file_name = filepath[slash_pos + 1: len(module_path)]
    else:
        file_name = filepath

    if parent_dir != '':
        slash_pos = parent_dir.rfind('/', 0, -1)
        if slash_pos != -1:
            parent_name = parent_dir[slash_pos + 1: len(parent_name) - 1]
        else:
            parent_name = parent_dir[0: len(parent_name) - 1]

    return parent_name


# ==============================================================================
# Executing Section


# ------------------------
# Script Environment
module_file = ''
module_path = os.path.abspath(__file__)
main_dir = ''
work_dir = ''


slash_pos = module_path.rfind('/', 0)

if slash_pos != -1:
    work_dir = module_path[0: slash_pos + 1]
    module_file = module_path[slash_pos + 1: len(module_path)]
else:
    module_file = module_path

if work_dir != '':
    slash_pos = work_dir.rfind('/', 0, -1)
    if slash_pos != -1:
        main_dir = work_dir[0: slash_pos + 1]
    else:
        main_dir = work_dir


# ------------------------
# Script Parameter

packages_search = []
module_output = 'plain'
module_debug = False
module_quiet = False
module_res = 0

for arg in sys.argv:
    if arg[0: 2] == '--':
        arg = arg[2: len(arg)]
        if arg in ['plain', 'json']:
            module_output = arg
        elif arg == 'debug':
            module_debug = True
        elif arg == 'quiet':
            module_quiet = True

    elif arg[0] == '-':
        arg = arg[1: len(arg)]
        for idx in range(0, len(arg)):
            if arg[idx] == 'd':
                module_debug = True
            elif arg[idx] == 'q':
                module_quiet = True
    else:
        if arg.rfind(module_file, 0) == -1:
            packages_search.append(arg)

if module_debug:
    print(
        "script '{}' - Version Searches:\n{}".format(module_file, str(packages_search)))
    print(
        "script '{}' - Search Output: '{}'".format(module_file, module_output))


# ------------------------
# Scan for Cargo ".toml" files

cargo_packages = {}
cargo_files = []
versions_res = {}

cargo_files = list_cargo_files(main_dir)

if module_debug:
    print("script '{}' - Cargo Files [{}]:\n{}".format(
        module_file, str(cargo_files['success']), str(cargo_files)))

if not cargo_files['success']:
    if not module_quiet:
        print("script '{}' - Cargo Files Search: Searching Files has failed!".format(
            module_file), file=sys.stderr)

    module_res = 1


# ------------------------
# Parse the Cargo files

cargo_packages = parse_cargo_files(cargo_files['files'])

if module_debug:
    print("script '{}' - Cargo Packages [{}]:\n{}".format(
        module_file, str(cargo_packages['success']), str(cargo_packages)))

if not cargo_packages['success']:
    if not module_quiet:
        print("script '{}' - Cargo Package Parsing: Parsing Packages has failed!".format(
            module_file), file=sys.stderr)

    module_res = 1

for search in packages_search:
    if search in cargo_packages['packages']:
        package_res = cargo_packages['packages'][search]

        if module_debug:
            print(
                "script '{}' - Package Hit:\n{}".format(module_file, str(package_res)))

        versions_res[search] = 0

        if 'package' in package_res:
            versions_res[search] = package_res['package']['version']

    else:
        if not module_quiet:
            print(
                "script '{}' - Package Search '{}': Look up Package failed!\nPackage '{}': Cargo.toml file not found!".format(
                    module_file, search, search),
                file=sys.stderr)

        module_res = 1

if module_debug:
    print(
        "script '{}' - Cargo Versions:\n{}".format(module_file, str(versions_res)))

if module_output == 'plain':
    print("script '{}' - Cargo Versions:".format(module_file))
    for search in versions_res:
        print("{}={}".format(search, versions_res[search]))

elif module_output == 'json':
    print("{}".format(json.dumps(versions_res)))

else:
    print(
        "script '{}' - Cargo Versions:\n{}".format(module_file, str(versions_res)))

if module_debug:
    print("script '{}': Script finished with [{}]".format(
        module_file, module_res))


sys.exit(module_res)
