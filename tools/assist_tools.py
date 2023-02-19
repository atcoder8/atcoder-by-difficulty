#!/usr/bin/env python

import argparse
from typing import Sequence

from _module import prepare, open, download, test, submit, remove
from _module.common import command


def _add_sub_parser(
    subparsers: argparse._SubParsersAction,
    command_class: type[command.Command],
    name: str,
    aliases: Sequence[str],
):
    parser = subparsers.add_parser(
        name,
        aliases=aliases,
        description=command_class.COMMAND_DESCRIPTION,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )

    command_class.add_arguments(parser)

    parser.set_defaults(
        run_command=lambda cmdline_args: command_class(cmdline_args).run_command()
    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser()

    subparsers = parser.add_subparsers()

    _add_sub_parser(subparsers, prepare.PrepareProblem, "prepare", ["p"])
    _add_sub_parser(subparsers, open.OpenProblemPage, "open", ["o"])
    _add_sub_parser(subparsers, download.DownloadTestcase, "download", ["d"])
    _add_sub_parser(subparsers, test.TestSolution, "test", ["t"])
    _add_sub_parser(subparsers, submit.SubmitSolution, "submit", ["s"])
    _add_sub_parser(subparsers, remove.RemoveSubmissionFile, "remove", ["r"])

    cmdline_args = parser.parse_args()

    assert hasattr(cmdline_args, "run_command")
    assert callable(cmdline_args.run_command)

    cmdline_args.run_command(cmdline_args)
