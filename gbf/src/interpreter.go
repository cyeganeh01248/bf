package main

import (
	"fmt"
	"io"
	"os"
)

type Instruction struct {
	inst uint8
	arg  int
}

const INCP_C = 1
const INCV_C = 2
const PRNT = 3
const INPT = 4
const JMP_C = 5

const JMPF = 8
const JMPB = 9

type Interpreter struct {
	program         []Instruction
	program_pointer int
	data            []uint8
	data_pointer    int
}

func NewInterpreter(program string) *Interpreter {
	cleaned_program := cleanProgram(program)
	if !validateProgram(cleaned_program) {
		panic("Invalid program")
	}

	insts := compileProgram(cleaned_program)

	interpreter := new(Interpreter)
	interpreter.program = insts
	interpreter.program_pointer = 0
	interpreter.data = make([]uint8, 30_000)
	interpreter.data_pointer = 0
	return interpreter
}

func (interpreter *Interpreter) Run() {
	for interpreter.program_pointer < len(interpreter.program) {
		// fmt.Println(interpreter.program_pointer)
		inst := interpreter.program[interpreter.program_pointer]
		switch inst.inst {
		case INCP_C:
			interpreter.data_pointer += inst.arg % len(interpreter.program)
		case INCV_C:
			interpreter.data[interpreter.data_pointer] = uint8((int(interpreter.data[interpreter.data_pointer]) + inst.arg + 256) % 256)
		case PRNT:
			fmt.Printf("%c", interpreter.data[interpreter.data_pointer])
		case INPT:
			buffer := make([]byte, 1)
			_, err := io.ReadFull(os.Stdin, buffer)
			if err != nil {
				fmt.Println("Error reading from stdin:", err)
				return
			}
			interpreter.data[interpreter.data_pointer] = buffer[0]
		case JMP_C:
			if (inst.arg > interpreter.program_pointer && interpreter.data[interpreter.data_pointer] == 0) ||
				(inst.arg < interpreter.program_pointer && interpreter.data[interpreter.data_pointer] != 0) {
				interpreter.program_pointer = inst.arg

			}
		}
		interpreter.program_pointer++
	}
}

func cleanProgram(program string) string {
	out := ""
	for _, char := range program {
		if char == '>' || char == '<' || char == '+' || char == '-' || char == '.' || char == ',' || char == '[' || char == ']' {
			out += string(char)
		}
	}
	return out
}
func validateProgram(program string) bool {
	d := 0
	for _, char := range program {
		if char == '[' {
			d++
		} else if char == ']' {
			d--
		}
		if d < 0 {
			return false
		}
	}
	return d == 0
}

func compileProgram(program string) []Instruction {
	intermediate := make([]Instruction, 0)
	i := 0
	for i < len(program) {
		char := program[i]
		switch char {
		case '>':
			j := i + 1
			for program[j] == '>' {
				j++
			}
			intermediate = append(intermediate, Instruction{inst: INCP_C, arg: j - i})
			i = j
		case '<':
			j := i + 1
			for program[j] == '<' {
				j++
			}
			intermediate = append(intermediate, Instruction{inst: INCP_C, arg: -(j - i)})
			i = j
		case '+':
			j := i + 1
			for program[j] == '+' {
				j++
			}
			intermediate = append(intermediate, Instruction{inst: INCV_C, arg: j - i})
			i = j
		case '-':
			j := i + 1
			for program[j] == '-' {
				j++
			}
			intermediate = append(intermediate, Instruction{inst: INCV_C, arg: -(j - i)})
			i = j
		case '.':
			intermediate = append(intermediate, Instruction{inst: PRNT, arg: 0})
			i++
		case ',':
			intermediate = append(intermediate, Instruction{inst: INPT, arg: 0})
			i++
		case '[':
			intermediate = append(intermediate, Instruction{inst: JMPF})
			i++
		case ']':
			intermediate = append(intermediate, Instruction{inst: JMPB})
			i++
		}
	}
	instructions := make([]Instruction, len(intermediate))
	for i := 0; i < len(intermediate); i++ {
		if intermediate[i].inst == JMPF {
			j := i + 1
			d := 0
			for j < len(intermediate) {
				if d == 0 && intermediate[j].inst == JMPB {
					break
				} else if intermediate[j].inst == JMPF {
					d++
				} else if intermediate[j].inst == JMPB {
					d--
				}
				j++
			}
			instructions[i].inst = JMP_C
			instructions[i].arg = j
		} else if intermediate[i].inst == JMPB {
			j := i - 1
			d := 0
			for j >= 0 {
				if d == 0 && intermediate[j].inst == JMPF {
					break
				} else if intermediate[j].inst == JMPB {
					d++
				} else if intermediate[j].inst == JMPF {
					d--
				}
				j--
			}
			instructions[i].inst = JMP_C
			instructions[i].arg = j
		} else {
			instructions[i] = intermediate[i]
		}
	}
	return instructions
}
