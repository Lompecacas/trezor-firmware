#!/usr/bin/env python3

import os
import re
import subprocess


def get_git_years(filepath):
    """Get the first and last commit year for a file using git."""
    try:
        first_commit = subprocess.check_output(
            [
                "git",
                "log",
                "--diff-filter=A",
                "--follow",
                "--format=%ad",
                "--date=format:%Y",
                filepath,
            ],
            text=True,
        ).splitlines()

        last_commit = subprocess.check_output(
            ["git", "log", "-1", "--format=%ad", "--date=format:%Y", filepath],
            text=True,
        ).strip()

        first_year = first_commit[0] if first_commit else "Unknown"
        last_year = last_commit if last_commit else "Unknown"
        return first_year, last_year
    except subprocess.CalledProcessError:
        return "Unknown", "Unknown"


def process_files():
    """Iterate over all files in the git repository and extract relevant license information."""
    repo_root = subprocess.check_output(
        ["git", "rev-parse", "--show-toplevel"], text=True
    ).strip()

    for root, _, files in os.walk(repo_root):
        for file in files:
            filepath = os.path.join(root, file)

            try:
                with open(filepath, "r", encoding="utf-8") as f:
                    lines = f.readlines()

                    if lines and lines[0].startswith(
                        "# This file is part of the Trezor project."
                    ):
                        copyright_years = re.findall(
                            r"Copyright \(C\) (\d{4})-(\d{4})", "".join(lines)
                        )

                        if copyright_years:
                            first_year, last_year = copyright_years[0]
                        else:
                            continue

                        git_first, git_last = get_git_years(filepath)

                        print(
                            f"{filepath}: {first_year}-{last_year}, Git First: {git_first}, Git Last: {git_last}"
                        )
            except (UnicodeDecodeError, FileNotFoundError, PermissionError):
                continue


def get_files_with_license():
    files_with_license = []
    files_without_license = []
    for subdir in subdirs:
        for root_dir, _, files in os.walk(subdir):
            for file in files:
                filepath = os.path.join(root_dir, file)

                try:
                    with open(filepath, "r", encoding="utf-8") as f:
                        line = f.readline()

                        if line and line.startswith(
                            "# This file is part of the Trezor project."
                        ):
                            files_with_license.append(filepath)
                        elif line:
                            files_without_license.append(filepath)
                except (UnicodeDecodeError, FileNotFoundError, PermissionError):
                    continue
    return files_with_license, files_without_license


subdirs = [
    # os.path.join("python", "build"),
    os.path.join("python", "src"),
    os.path.join("python", "tests"),
    "tests",
]

if __name__ == "__main__":
    licensed, unlicensed = get_files_with_license()
    for f in unlicensed:
        if f.endswith(".py"):
            print(f)
    # for f in licensed:
    #     first_commit = subprocess.check_output(
    #         [
    #             "git",
    #             "log",
    #             "--diff-filter=A",
    #             "--follow",
    #             "--format=%ad",
    #             "--date=format:%Y",
    #             f,
    #         ],
    #         text=True,
    #     ).splitlines()
    #     second = subprocess.check_output(
    #         [
    #             "git",
    #             "log",
    #             "--diff-filter=A",
    #             "--format=%ad",
    #             "--date=format:%Y",
    #             f,
    #         ],
    #         text=True,
    #     ).splitlines()
    #     last = subprocess.check_output(
    #         [
    #             "git",
    #             "log",
    #             "-1",
    #             "--format=%ad",
    #             "--date=format:%Y",
    #             f,
    #         ],
    #         text=True,
    #     ).strip()
    #     print(f, second, last)
