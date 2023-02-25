#!/usr/bin/python3

# @author Bodo (Hugo) Barwich
# @version 2023-02-21
# @package TextSanitizer
# @subpackage scripts/cargo_version.py

# This Module parses the Git History to find the Merge Commit for a given Commit Hash
#

import sys
import os
import os.path
import toml
import json
from git import Repo, GitCommandError


# ==============================================================================
# Auxiliary Functions

def git_history(module_name, git):
    history_result = {'success': True, 'commits': []}
    history_info = None
    history_lines = []

    try:
        history_info = git.log('-50')
    except GitCommandError as e:
        if not module_quiet:
            print(
                "script '{}' - Cargo File '{}': Blame File failed!".format(
                    module_name, filepath), file=sys.stderr)
            print("script '{}' - Cargo File Exception Message: {}".format(
                module_name, str(e)), file=sys.stderr)

        history_result['success'] = False

    if module_debug:
        print("history info:'{}'".format(history_info))

    if history_info is not None:
        history_lines = history_info.split("\n")

    commit_idx = 0

    for line in history_lines:
        if line[0: 6] == 'commit':
            commit_hash = line.removeprefix('commit ')
            commit = {'hash': commit_hash,
                      'hash_short': commit_hash[0: 7],
                      'raw': [],
                      'index': commit_idx}
            commit['raw'].append(line)

            if module_debug:
                print("commit dict:'{}'".format(str(commit)))

            history_result['commits'].append(commit)

            commit_idx += 1

        else:
            commit['raw'].append(line)
            colon_pos = line.find(':')

            if colon_pos != -1:
                line_key = line[0: colon_pos].lower()
                line_value = line[colon_pos + 1: len(line)].lstrip()
                author_info = None
                merge_info = None

                if line_key == 'author':
                    author_info = parse_commit_author(line_value)
                elif line_key == 'merge':
                    merge_commits = line_value.split(' ')
                    merge_info = {
                        'dest': merge_commits[0],
                        'origin': merge_commits[1]}

                if author_info is not None:
                    commit[line_key] = author_info
                elif merge_info is not None:
                    commit[line_key] = merge_info
                else:
                    commit[line_key] = line_value

    return history_result


def parse_commit_author(author_value):
    author = None
    email_pos = author_value.find('<')

    if email_pos != -1:
        author = {'name': author_value[0: email_pos - 1].rstrip(),
                  'email': author_value[email_pos + 1: len(author_value)].rstrip('>')}

    return author


def find_merge_commit(history, commit):
    commit_idx = len(history) - 1
    commit_search = None
    commit_merge = None

    if 'index' in commit:
        commit_idx = commit['index']

    while commit_idx >= 0 and commit_merge is None:
        commit_search = history[commit_idx]

        # print("commit search: '{}'".format(str(commit_search)))

        if 'merge' in commit_search:
            print("commit search: HIT")
            commit_merge = commit_search
        else:
            commit_idx -= 1

    return commit_merge


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

commit_search = []
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
            if len(arg) > 7:
                commit_search.append(arg[0: 7])
            else:
                commit_search.append(arg)

if module_debug:
    print(
        "script '{}' - Commit Searches:\n{}".format(module_file, str(commit_search)))
    print(
        "script '{}' - Search Output: '{}'".format(module_file, module_output))

if len(commit_search) == 0:
    print(
        "script '{}' - Commit Hash is missing!".format(module_file))

    module_res = 3

# ------------------------
# Parse the Git History


repo = Repo('.git')
git = repo.git

history_result = git_history(module_file, git)
history_commits = {}
commits_res = {}

if module_debug:
    print(
        "script '{}' - History Result:\n{}".format(module_file, str(history_result)))

if not history_result['success']:
    if not module_quiet:
        print("script '{}' - Git History Parsing: Parsing History has failed!".format(
            module_file), file=sys.stderr)

    module_res = 1

for commit in history_result['commits']:
    if 'hash_short' in commit:
        history_commits[commit['hash_short']] = commit

for search in commit_search:
    commits_res[search] = 0

    if search in history_commits:
        commits_res[search] = {
            'origin': history_commits[search],
            'merge': find_merge_commit(
                history_result['commits'],
                history_commits[search])}

# ------------------------
# Print the Commit Result

if module_output == 'plain':
    if len(commits_res) > 0:
        print("script '{}' - Version Commits:".format(module_file))

        for search in commits_res:
            if commits_res[search] != 0:
                if commits_res[search]['merge'] is not None:
                    print(
                        "{}/{} by '{}/{}'".format(
                            commits_res[search]['merge']['hash_short'],
                            commits_res[search]['merge']['hash'],
                            commits_res[search]['merge']['author']['name'],
                            commits_res[search]['merge']['author']['email']))
                else:
                    print("{}/{} by '{}/{}'".format(search,
                                                    commits_res[search]['origin']['hash'],
                                                    commits_res[search]['origin']['author']['name'],
                                                    commits_res[search]['origin']['author']['email']))

            else:
                print("{} - no entry found".format(search))

                module_res = 1

elif module_output == 'json':
    print("{}".format(json.dumps(commits_res)))

else:
    print(
        "script '{}' - Version Commits:\n{}".format(module_file, str(commits_res)))


if module_debug:
    print("script '{}': Script finished with [{}]".format(
        module_file, module_res))


sys.exit(module_res)
