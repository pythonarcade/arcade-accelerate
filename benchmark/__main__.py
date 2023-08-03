import multiprocessing
import argparse
import sys
from datetime import datetime

from benchmark.manager import TestManager


def unload_arcade():
    to_uncache = []
    for mod in sys.modules:
        if mod.startsWith("arcade."):
            to_uncache.append(mod)

    for mod in to_uncache:
        del sys.modules[mod]


def main():
    args = parse_args(sys.argv[1:])
    print(f"Session Name: '{args.session}'")
    manager = TestManager(args.session, debug=True)
    manager.find_test_classes(args.type, args.name)
    manager.create_test_instances()
    manager.run()


def parse_args(args):
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-s",
        "--session",
        help="Session Name",
        type=str,
        default=datetime.now().strftime("%Y-%m-%dT%H-%M-%S"),
    )
    parser.add_argument("-t", "--type", help="Test Type", type=str)
    parser.add_argument("-n", "--name", help="Test Name", type=str)
    return parser.parse_args(args)


if __name__ == "__main__":
    multiprocessing.set_start_method("spawn")
    main()
