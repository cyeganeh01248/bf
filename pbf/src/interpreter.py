from enum import Enum
from sys import stdin


class Interpreter:
    def __init__(self, program: str) -> None:
        cleaned_program = Interpreter.clean_program(program)
        if not Interpreter.validate_program(cleaned_program):
            raise ValueError("Invalid program.")
        self.program = Interpreter.compile_program(cleaned_program)
        self.program_pointer = 0
        self.data = [0 for _ in range(30_000)]
        self.data_pointer = 0

    def run(self):
        while self.program_pointer < len(self.program):
            (inst, arg) = self.program[self.program_pointer]
            match inst:
                case Instruction.INCP_C:
                    self.data_pointer = (self.data_pointer + arg) % len(
                        self.data
                    )
                case Instruction.INCV_C:
                    self.data[self.data_pointer] = (
                        self.data[self.data_pointer] + arg + 256
                    ) % 256
                case Instruction.PRNT:
                    print(chr(self.data[self.data_pointer]), end="")
                case Instruction.INPT:
                    self.data[self.data_pointer] = ord(stdin.buffer.read(1))
                case Instruction.JUMP_C:
                    if (
                        arg < self.program_pointer
                        and self.data[self.data_pointer] != 0
                    ) or (
                        arg > self.program_pointer
                        and self.data[self.data_pointer] == 0
                    ):
                        self.program_pointer = arg
                case _:
                    pass
            self.program_pointer += 1

    @staticmethod
    def clean_program(program: str) -> str:
        out: list[str] = []
        for char in program:
            if char in Interpreter.VALID_CHARS:
                out += [char]
        return "".join(out)

    @staticmethod
    def validate_program(program: str) -> bool:
        d = 0
        for c in program:
            if c == "[":
                d += 1
            elif c == "]":
                d -= 1
            if d < 0:
                return False
        return d == 0

    VALID_CHARS = [">", "<", "+", "-", ".", ",", "[", "]"]

    @staticmethod
    def compile_program(program: str) -> list[tuple["Instruction", int]]:
        intermediate: list[tuple["Instruction", int]] = []
        i = 0
        while i < len(program):
            c = program[i]
            if c == ">":
                j = i + 1
                while program[j] == ">":
                    j += 1
                d = j - i
                intermediate += [(Instruction.INCP_C, d)]
                i += d
            elif c == "<":
                j = i + 1
                while program[j] == "<":
                    j += 1
                d = j - i
                intermediate += [(Instruction.INCP_C, -d)]
                i += d
            elif c == "+":
                j = i + 1
                while program[j] == "+":
                    j += 1
                d = j - i
                intermediate += [(Instruction.INCV_C, d)]
                i += d
            elif c == "-":
                j = i + 1
                while program[j] == "-":
                    j += 1
                d = j - i
                intermediate += [(Instruction.INCV_C, -d)]
                i += d
            elif c == ".":
                intermediate += [(Instruction.PRNT, 0)]
                i += 1
            elif c == ",":
                intermediate += [(Instruction.INPT, 0)]
                i += 1
            elif c == "[":
                intermediate += [(Instruction.JMPF, 0)]
                i += 1
            elif c == "]":
                intermediate += [(Instruction.JMPB, 0)]
                i += 1
        out: list[tuple[Instruction, int]] = []
        for i in range(0, len(intermediate)):
            if intermediate[i][0] == Instruction.JMPF:
                j = i + 1
                d = 0
                while j < len(intermediate):
                    if d == 0 and intermediate[j][0] == Instruction.JMPB:
                        break
                    elif intermediate[j][0] == Instruction.JMPB:
                        d -= 1
                    elif intermediate[j][0] == Instruction.JMPF:
                        d += 1
                    j += 1
                out += [(Instruction.JUMP_C, j)]
            elif intermediate[i][0] == Instruction.JMPB:
                j = i - 1
                d = 0
                while j >= 0:
                    if d == 0 and intermediate[j][0] == Instruction.JMPF:
                        break
                    elif intermediate[j][0] == Instruction.JMPF:
                        d -= 1
                    elif intermediate[j][0] == Instruction.JMPB:
                        d += 1
                    j -= 1
                out += [(Instruction.JUMP_C, j)]
            else:
                out += [intermediate[i]]
        return out


class Instruction(Enum):
    NOOP = 1

    INCP_C = 2

    INCV_C = 3

    PRNT = 4

    INPT = 5

    JUMP_C = 6

    JMPF = 10
    JMPB = 11
