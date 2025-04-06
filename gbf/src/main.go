package main

import (
	"os"
)

func main() {
	file, err := os.ReadFile(os.Args[1])
	if err != nil {
		panic("File not found.")
	}
	program := string(file[:])
	interpreter := NewInterpreter(program)
	interpreter.Run()
}
