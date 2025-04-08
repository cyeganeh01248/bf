local Interpreter = require("inter")
describe("Test", function()
	it("should work", function()
		local fp = io.open("../examples/abc.bf", "r")
		if fp == nil then
			error("File not found: " .. "../examples/abc.bf")
		end
		local program = fp:read("a")
		fp:close()

		local i = Interpreter(program)
		i:exec()
	end)
	it("should work 2", function()
		assert.has_error(function()
			local i = Interpreter("]")
			i:exec()
		end)
	end)
end)
