file = "../INPUT/06"

fish = {}

for i = 1,9,1 do
    table.insert(fish, 0)
end

io.input(file)
input = io.read("*all")

for timer in input:gmatch("[0-9]+") do
    timer = tonumber(timer) + 1
    fish[timer] = fish[timer] + 1
end

function sim_day()
    local new_fish = {}
    for i = 1,9,1 do
        table.insert(new_fish, 0)
    end

    for i = 1,#fish,1 do
        new_fish[9] = fish[1]

        for j = 8,1,-1 do
            new_fish[j] = fish[j+1]
        end

        new_fish[7] = new_fish[7] + fish[1]
    end

    fish = new_fish
end

function count_fish()
    local count = 0

    for i = 1,#fish,1 do
        count = count + fish[i]
    end

    return count
end

for i = 1,80,1 do
    sim_day()
end

print("part1:", count_fish(fish))

for i = 1,(256-80),1 do
    sim_day()
end

print("part2:", count_fish(fish))
