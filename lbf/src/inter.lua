---@class Interpreter
---@class Instruction

local class = require("classy")
local Interpreter = class("Interpreter")
local Instruction = class("Instruction")

local NOOP = 0

local INCP_C = 1

local INCV_C = 2

local PRNT = 3

local INPT = 4

local JUMP_C = 5

local JMPF = 10
local JMPB = 11

function Instruction:__init(type, op)
	self.type = type
	self.op = op
end

---@param program string
function Interpreter:__init(program)
	local cleaned_program = Interpreter.clean_program(program)

	if not Interpreter.check_valid(cleaned_program) then
		error("Invalid program")
	end

	local compiled_program = Interpreter.compile_program(cleaned_program)
	self.data = {}
	for i = 0, (30000 - 1) do
		self.data[i] = 0
	end
	self.data_pointer = 0
	self.program_pointer = 1
	self.program = compiled_program
end

function Interpreter:exec()
	while self.program_pointer <= #self.program do
		local inst = self.program[self.program_pointer]
		if inst.type == INCP_C then
			self.data_pointer = (self.data_pointer + inst.op + #self.data) % #self.data
		elseif inst.type == INCV_C then
			self.data[self.data_pointer] = (self.data[self.data_pointer] + 256 + inst.op) % 256
		elseif inst.type == PRNT then
			io.write(string.char(self.data[self.data_pointer]))
		elseif inst.type == INPT then
			local t = io.read(1)
			self.data[self.data_pointer] = string.byte(t)
		elseif inst.type == JUMP_C then
			local dest = inst.op
			if
				(dest < self.program_pointer and self.data[self.data_pointer] ~= 0)
				or (dest > self.program_pointer and self.data[self.data_pointer] == 0)
			then
				self.program_pointer = dest
			end
		else
		end
		self.program_pointer = self.program_pointer + 1
	end
	print()
end

---@param program string
---@return Instruction[]
function Interpreter.compile_program(program)
	local out = {}

	local i = 1
	while i <= program:len() do
		local c = program:sub(i, i)
		if c == ">" then
			local count = 1
			for j = (i + 1), program:len() do
				if program:sub(j, j) ~= ">" then
					break
				end
				count = count + 1
			end
			i = i + count
			table.insert(out, Instruction(INCP_C, count))
		elseif c == "<" then
			local count = 1
			for j = (i + 1), program:len() do
				if program:sub(j, j) ~= "<" then
					break
				end
				count = count + 1
			end
			i = i + count
			table.insert(out, Instruction(INCP_C, -count))
		elseif c == "+" then
			local count = 1
			for j = (i + 1), program:len() do
				if program:sub(j, j) ~= "+" then
					break
				end
				count = count + 1
			end
			i = i + count
			table.insert(out, Instruction(INCV_C, count))
		elseif c == "-" then
			local count = 1
			for j = (i + 1), program:len() do
				if program:sub(j, j) ~= "-" then
					break
				end
				count = count + 1
			end
			i = i + count
			table.insert(out, Instruction(INCV_C, -count))
		elseif c == "." then
			i = i + 1
			table.insert(out, Instruction(PRNT))
		elseif c == "," then
			i = i + 1
			table.insert(out, Instruction(INPT))
		elseif c == "[" then
			i = i + 1
			table.insert(out, Instruction(JMPF))
		elseif c == "]" then
			i = i + 1
			table.insert(out, Instruction(JMPB))
		else
			i = i + 1
		end
	end
	local final_out = {}
	for j = 1, #out do
		local inst = out[j]
		if inst.type == JMPF then
			local d = 0
			for k = j + 1, #out do
				if out[k].type == JMPB and d == 0 then
					table.insert(final_out, Instruction(JUMP_C, k))
					break
				elseif out[k].type == JMPB then
					d = d - 1
				elseif out[k].type == JMPF then
					d = d + 1
				end
			end
		elseif inst.type == JMPB then
			local d = 0
			for k = j - 1, 1, -1 do
				if out[k].type == JMPF and d == 0 then
					table.insert(final_out, Instruction(JUMP_C, k))
					break
				elseif out[k].type == JMPF then
					d = d - 1
				elseif out[k].type == JMPB then
					d = d + 1
				end
			end
		else
			table.insert(final_out, inst)
		end
	end
	return final_out
end

Interpreter.valid_chars = { [">"] = 1, ["<"] = 1, ["+"] = 1, ["-"] = 1, ["."] = 1, [","] = 1, ["["] = 1, ["]"] = 1 }

---@param program string
---@return boolean
function Interpreter.check_valid(program)
	local depth = 0
	for i = 1, #program do
		local c = program:sub(i, i)
		if c == "[" then
			depth = depth + 1
		elseif c == "]" then
			depth = depth - 1
		end

		if depth < 0 then
			return false
		end
	end

	return depth == 0
end

---@param program string
---@return string
function Interpreter.clean_program(program)
	local out = ""
	for i = 1, program:len() do
		local c = program:sub(i, i)
		if Interpreter.valid_chars[c] then
			out = out .. c
		end
	end
	return out
end

return Interpreter
