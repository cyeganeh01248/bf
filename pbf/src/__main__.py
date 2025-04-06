from argparse import ArgumentParser

from interpreter import Interpreter


def main():
    parser = ArgumentParser()
    parser.add_argument("program")
    args = parser.parse_args()
    program = ""
    with open(args.program, "r") as f:
        program = f.read()
    interpreter = Interpreter(program)
    interpreter.run()


if __name__ == "__main__":
    main()
