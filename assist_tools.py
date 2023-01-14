#!/usr/bin/env python

import argparse
import subprocess
import re
import shutil
import os
import sys
from urllib import request


PROBLEM_NUMBER_PATTERN = re.compile("^(a[brg]c)([0-9]{3})-([a-h]|(ex))$")

OJ_PATH = "./.venv/bin/oj"
TESTCASE_DIR_PATHNAME = "./test"
BIN_TARGET_FILE_FORMAT = "./{contest_class}-{problem_number}/src/bin/{contest_class}{contest_number:03d}-{problem_number}.rs"

ProblemNumberBoundary = {"abc": 20, "arc": 35, "agc": 1}


class AbnormalEndException(Exception):
    pass


class InvalidContestNameException(Exception):
    pass


class ProblemNameParts:
    _PROBLEM_NAME_PATTERN = re.compile("^(a[brg]c)([0-9]{3})-([a-h]|(?:ex))$")
    _PROBLEM_URL_FORMAT = "https://atcoder.jp/contests/{contest_name}/tasks/{contest_name}_{problem_number}"
    _BIN_TARGET_FILE_FORMAT = "./{contest_class}-{problem_number}/src/bin/{contest_class}{contest_number:03d}-{problem_number}.rs"
    _TESTCASE_DIR_PATHNAME = "./test"
    _OJ_PATH = "./.venv/bin/oj"

    def __init__(self, problem_name: str) -> None:
        match_obj = self._PROBLEM_NAME_PATTERN.fullmatch(problem_name)

        if match_obj is None:
            raise InvalidContestNameException()

        self.problem_name = problem_name

        split_problem_name = problem_name.split("-")

        self.contest_name = split_problem_name[0]
        self.problem_number = split_problem_name[1]
        self.contest_class = self.contest_name[:3]
        self.contest_number = int(self.contest_name[3:])

    def problem_url(self) -> str:
        if self.contest_number < ProblemNumberBoundary[self.contest_class]:
            problem_number_for_url = str(ord(self.problem_number) - ord("a") + 1)
        else:
            problem_number_for_url = self.problem_number

        return self._PROBLEM_URL_FORMAT.format(
            contest_name=self.contest_name, problem_number=problem_number_for_url
        )

    def bin_target_file_pathname(self) -> str:
        return self._BIN_TARGET_FILE_FORMAT.format(
            contest_class=self.contest_class,
            contest_number=self.contest_number,
            problem_number=self.problem_number,
        )

    def open_url(self) -> subprocess.CompletedProcess[bytes]:
        args = ["xdg-open", self.problem_url()]
        completed_process = subprocess.run(args)

        return completed_process

    def update_testcase(self) -> subprocess.CompletedProcess[bytes]:
        if os.path.exists(TESTCASE_DIR_PATHNAME):
            shutil.rmtree(TESTCASE_DIR_PATHNAME)

        args = [self._OJ_PATH, "download", self.problem_url()]
        completed_process = subprocess.run(args)

        return completed_process

    def sample_test(self) -> subprocess.CompletedProcess[bytes]:
        test_args = [OJ_PATH, "test", "-c", f"cargo run --bin {self.problem_name}"]
        completed_process = subprocess.run(test_args)

        return completed_process

    def submit(self) -> subprocess.CompletedProcess[bytes]:
        submit_args = [
            OJ_PATH,
            "submit",
            "--yes",
            "-w",
            "0",
            self.problem_url(),
            self.bin_target_file_pathname(),
        ]
        completed_process = subprocess.run(submit_args)

        return completed_process


def print_error_message(
    completed_process: subprocess.CompletedProcess[bytes], file=sys.stderr
) -> None:
    print(
        f"""\
Terminated with a non-zero exit code {completed_process.returncode}.

[Standard output]
{str(completed_process.stdout)}

[Standard error]
{str(completed_process.stderr)}\
""",
        file=sys.stderr,
    )


def run_open_url(cmdline_args: argparse.Namespace):
    problem_name: str = cmdline_args.problem_name
    print_url: bool = cmdline_args.print_url

    problem_name_parts = ProblemNameParts(problem_name)
    problem_url = problem_name_parts.problem_url()

    if print_url:
        print(problem_url)
    else:
        completed_process = problem_name_parts.open_url()

        if completed_process.returncode != 0:
            raise AbnormalEndException("Failed to open URL.")


def run_submit(cmdline_args: argparse.Namespace):
    cmdline_args = parser.parse_args()

    problem_name: str = cmdline_args.problem_name
    keep_testcase: bool = cmdline_args.keep_testcase
    no_test: bool = cmdline_args.no_test

    problem_name_parts = ProblemNameParts(problem_name)

    if not keep_testcase and not no_test:
        completed_process = problem_name_parts.update_testcase()

        if completed_process.returncode != 0:
            raise AbnormalEndException("Failed to download testcase.")

    if not no_test:
        completed_process = problem_name_parts.sample_test()

        if completed_process.returncode != 0:
            raise AbnormalEndException("Failed sample test.")

    completed_process = problem_name_parts.submit()

    if completed_process.returncode != 0:
        raise AbnormalEndException("Failed to submit the program.")


def add_arguments_for_parser_for_open_url(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        "problem_name", type=str, help="Name of problem. (Example: abc001-a)"
    )

    parser.add_argument(
        "-p",
        "--print-url",
        action="store_true",
        help="If this option is specified,"
        " the URL is printed to standard output rather than opened.",
    )

    parser.set_defaults(run=run_open_url)


def add_arguments_for_parser_for_submit(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        "problem_name", type=str, help="Name of problem. (Example: abc001-a)"
    )

    parser.add_argument(
        "-k",
        "--keep-testcase",
        action="store_true",
        help="If this option is specified, previously downloaded test cases will be maintained.",
    )

    parser.add_argument(
        "--no-test",
        action="store_true",
        help="If this option is specified, submit without testing.",
    )

    parser.set_defaults(run=run_submit)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()

    subparsers = parser.add_subparsers()

    parser_for_open_url = subparsers.add_parser(
        "url", aliases=["u"], description="Opens the url."
    )

    add_arguments_for_parser_for_open_url(parser_for_open_url)

    parser_for_submit = subparsers.add_parser(
        "submit", aliases=["s"], description="Submit program."
    )

    add_arguments_for_parser_for_submit(parser_for_submit)

    cmdline_args = parser.parse_args()

    if hasattr(cmdline_args, "run"):
        cmdline_args.run(cmdline_args)
    else:
        parser.print_help()
        exit(1)
