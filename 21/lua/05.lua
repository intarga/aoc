file = "../INPUT/05"

grid = {}

function inc(x, y)
    if grid[x] == nil then
        grid[x] = {}
    end
    if grid[x][y] == nil then
        grid[x][y] = 1
    else
        grid[x][y] = grid[x][y] + 1
    end
end

for line in io.lines(file) do
    local coords = line:gmatch("([0-9]+)")
    local x1 = tonumber(coords())
    local y1 = tonumber(coords())
    local x2 = tonumber(coords())
    local y2 = tonumber(coords())

    if x1 == x2 then
        local x = x1
        local delta
        if y1 > y2 then delta = -1 else delta = 1 end

        for y = y1,y2,delta do
            inc(x,y)
        end
    elseif y1 == y2 then
        local y = y1
        local delta
        if x1 > x2 then delta = -1 else delta = 1 end

        for x = x1,x2,delta do
            inc(x,y)
        end
    end
end

function count_crosses()
    local count = 0

    for _, ys in pairs(grid) do
        for _, spot in pairs(ys) do
            if spot >= 2 then count = count + 1 end
        end
    end

    return count
end

print("part1:", count_crosses())

for line in io.lines(file) do
    local coords = line:gmatch("([0-9]+)")
    local x1 = tonumber(coords())
    local y1 = tonumber(coords())
    local x2 = tonumber(coords())
    local y2 = tonumber(coords())

    if x1 ~= x2 and y1 ~= y2  then
        local x = x1
        local xdelta, ydelta
        if x1 > x2 then xdelta = -1 else xdelta = 1 end
        if y1 > y2 then ydelta = -1 else ydelta = 1 end

        local y = y1
        for x = x1,x2,xdelta do
            inc(x,y)
            y = y + ydelta
        end
    end
end

print("part2:", count_crosses())
