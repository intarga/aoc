local file = "../INPUT/02"

local hor = 0
local dep = 0

for line in io.lines(file) do
    local match = string.gmatch(line, "%S+")
    local dir = match()
    local amp = match()

    -- print("dir:", dir, "amp:", amp)
    if dir == "forward" then
        hor = hor + amp
    elseif dir == "down" then
        dep = dep + amp
    else
        dep = dep - amp
    end
end

print("part1:", hor * dep)

hor = 0
dep = 0
local aim = 0

for line in io.lines(file) do
    local match = string.gmatch(line, "%S+")
    local dir = match()
    local amp = match()

    if dir == "forward" then
        hor = hor + amp
        dep = dep + (amp * aim)
    elseif dir == "down" then
        aim = aim + amp
    else
        aim = aim - amp
    end
end

print("part2:", hor * dep)
