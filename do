#!/usr/bin/env python3
import sys
import os

args = sys.argv

script_name = args.pop(0)


def usage():
    print(script_name, "( test | coverage ) [...]")
    exit(1)


if len(args) == 0:
    usage()

subcommand = args.pop(0)


def split_by_double_dash(array):
    i = array.index("--") if "--" in array else len(array)
    return array[:i], array[i + 1 :]


def exec(*args):
    program, args = args[0], args[1:]
    os.execlp(program, program, *args)

match subcommand:
    case "test":
        args, rest = split_by_double_dash(args)
        exec("cargo", "test", *args, "--", "--test-threads", "1", *rest)

    case "coverage":
        args, rest = split_by_double_dash(args)
        exec("cargo", "llvm-cov", *args, "--", "--test-threads", "1", *rest)

    case "tests": pass

    case _:
        usage()
