local Interpreter = require("inter")

local program

local fp = io.open(arg[1], "r")
if fp == nil then
	error("File not found: " .. arg[1])
end
program = fp:read("a")
fp:close()

local i = Interpreter(program)
i:exec()
