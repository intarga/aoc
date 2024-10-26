local file = "../INPUT/01"

local increases = -1
local depth = 0

for line in io.lines(file) do
    line = tonumber(line)
    if depth < line then
        increases = increases + 1
    end
    depth = line
end

print("part1:", increases)

local depths = {}
increases = 0

for line in io.lines(file) do
    line = tonumber(line)
    if #depths < 3 then
        table.insert(depths, line)
    else
        --[[ print(depths)
        for i, v in ipairs(depths) do
            print("depth:", i, v)
        end
        print("line:", line) ]]

        local currdepths = depths[1] + depths[2] + depths[3]
        local nextdepths = depths[2] + depths[3] + line
        --[[ print("currdepths:", currdepths)
        print("nextdepths:", nextdepths) ]]

        if nextdepths > currdepths then increases = increases +1 end

        table.remove(depths, 1)
        table.insert(depths, line)
    end
end

print("part2:", increases)
