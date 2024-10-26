file = "../INPUT/08"

input = {}

for line in io.lines(file) do
    local sigs = line:gmatch("[a-z]+")

    local p1 = {}
    for i = 1,10,1 do
        table.insert(p1, sigs())
    end
    local p2 = {}
    for i = 1,4,1 do
        table.insert(p2, sigs())
    end

    table.insert(input, {p1, p2})
end

--[[ for _, line in ipairs(input) do
local s1 = ""
for _, sig in ipairs(line[1]) do
    s1 = s1 .. ' ' .. sig
end
local s2 = ""
for _, sig in ipairs(line[2]) do
    s2 = s2 .. ' ' .. sig
end

print(s1)
print(s2)
end ]]

function uniq_len(s)
    local len = string.len(s)
    if len == 2 or len == 4 or len == 3 or len == 7 then
        return true
    end
    return false
end

uniqs = 0
for _, line in ipairs(input) do
    for _, sig in ipairs(line[2]) do
        if uniq_len(sig) then
            uniqs = uniqs +1
        end
    end
end

print("part1:", uniqs)
