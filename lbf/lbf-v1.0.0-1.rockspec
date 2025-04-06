package = "lbf"
version = "v1.0.0-1"
source = {
    url = "https://onedev.wackywombat.xyz/bf/lbf"
}
description = {
    license = "MIT",
    summary = "A Brainfuck interpreter written in lua."
}
build = {
    type = "builtin",
    modules = {
        main = "src/main.lua"
    }
}
dependencies = {
    "classy == 0.4-1"
}
