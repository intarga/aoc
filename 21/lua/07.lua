file = "../INPUT/07"
-- file = "../INPUT/07_test"

io.input(file)
input = io.read("*all")

crabs = {}

for pos in input:gmatch("[0-9]+") do
    table.insert(crabs, tonumber(pos))
end

highest_pos = 0
for _, pos in ipairs(crabs) do
    if pos > highest_pos then highest_pos = pos end
end

function find_fuel(align)
    local spent_fuel = 0
    for _, pos in ipairs(crabs) do
        spent_fuel = spent_fuel + math.abs(pos - align)
    end
    return spent_fuel
end

lowest_fuel = find_fuel(0)
for i = 1,highest_pos, 1 do
    local newfuel = find_fuel(i)
    if newfuel < lowest_fuel then lowest_fuel = newfuel end
end

print("part1:", lowest_fuel)

function fact_sum(dist)
    local fuel = 0
    for i = 1,dist,1 do
        fuel = fuel + i
    end
    return fuel
end

function find_fuel_2(align)
    local spent_fuel = 0
    for _, pos in ipairs(crabs) do
        spent_fuel = spent_fuel + fact_sum(math.abs(pos - align))
    end
    return spent_fuel
end

lowest_fuel = find_fuel_2(0)
for i = 1,highest_pos, 1 do
    local newfuel = find_fuel_2(i)
    if newfuel < lowest_fuel then lowest_fuel = newfuel end
end

print("part2:", lowest_fuel)
