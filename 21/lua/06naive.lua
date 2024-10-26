file = "../INPUT/06"

fish = {}

io.input(file)
input = io.read("*all")

for timer in input:gmatch("[0-9]+") do
    table.insert(fish, tonumber(timer))
end

function sim_day()
    for i = 1,#fish,1 do
        if fish[i] > 0 then
            fish[i] = fish[i] - 1
        else
            table.insert(fish, 8)
            fish[i] = 6
        end
    end
end

for i = 1,80,1 do
    sim_day()
end

print("part1:", #fish)

for i = 1,(256-80),1 do
    sim_day()
end

print("part2:", #fish)
