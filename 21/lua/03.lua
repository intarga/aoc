local function count_digits(file)
    local acc = {}

    for line in io.lines(file) do
        for i = 1, string.len(line) do
            local digit = string.sub(line, i, i)
            local delta = (digit * 2) - 1

            if i > #acc then table.insert(acc, 0) end

            acc[i] = acc[i] + delta
        end
    end

    return acc
end

local function reduce_digits(acc)
    for i = 1, #acc do
        if acc[i] > 0 then
            acc[i] = 1
        else
            acc[i] = 0
        end
    end

    return acc
end

local function get_gamma(acc)
    local gamma = 0

    for i = 1, #acc do
        gamma = gamma + (acc[i] * 2 ^ (#acc - i))
    end

    return gamma
end

local function get_epsilon(acc)
    local epsilon = 0

    for i = 1, #acc do
        epsilon = epsilon + (((acc[i] - 1) * -1) * 2 ^ (#acc - i))
    end

    return epsilon
end

local acc = reduce_digits(count_digits("../INPUT/03"))
local gamma = get_gamma(acc)
local epsilon = get_epsilon(acc)

print("part1:", gamma * epsilon)

function read_to_table(file)
    local t = {}

    for line in io.lines(file) do
        table.insert(t, line)
    end

    return t
end

function reduce_entries(t, delete_rule)
    local i = 1

    while #t > 1 do
        local acc = 0

        for j = 1, #t do
            digit = string.sub(t[j], i, i)
            acc = acc + (tonumber(digit) * 2) - 1
        end

        local delete_on = delete_rule(acc)

        for j = #t, 1, -1 do
            digit = tonumber(string.sub(t[j], i, i))

            if digit == delete_on then
                table.remove(t, j)
            end
        end

        i = i + 1
    end

    return t[1]
end

function bin_to_dec(b)
    local dec = 0

    for i = 1, string.len(b) do
        dec = dec + (string.sub(b, i, i) * 2 ^ (string.len(b) - i))
    end

    return dec
end

local o2rule = function(acc)
    local delete_on = 0
    if acc < 0 then delete_on = 1 end

    return delete_on
end

local co2rule = function(acc)
    local delete_on = 0
    if acc >= 0 then delete_on = 1 end

    return delete_on
end

local o2 = bin_to_dec(reduce_entries(read_to_table("../INPUT/03"), o2rule))
local co2 = bin_to_dec(reduce_entries(read_to_table("../INPUT/03"), co2rule))

print("part2:", o2 * co2)

