file = "../INPUT/04"

function read_draws(file)
    local lines = io.lines(file)

    local draws_raw = lines()
    local draws = {}

    for draw in draws_raw:gmatch("([0-9]+)") do
        table.insert(draws, tonumber(draw))
    end

    return draws
end

function read_boards(file)
    local lines = io.lines(file)
    lines()

    local boards = {}

    for line in lines do
        local y = 0
        local board = {}

        while y < 5 do
            local row = {}

            line = lines()

            for sq in line:gmatch("([0-9]+)") do
                table.insert(row, {tonumber(sq), false})
            end

            table.insert(board, row)
            y = y + 1
        end

        table.insert(boards, board)
    end

    return boards
end

function check_rows(board)
    for i = 1, 5, 1 do
        local bingo = true
        for j = 1, 5, 1 do
            if not board[i][j][2] then
                bingo = false
                break
            end
        end
        if bingo then return true end
    end
end

function check_cols(board)
    for i = 1, 5, 1 do
        local bingo = true
        for j = 1, 5, 1 do
            if not board[j][i][2] then
                bingo = false
                break
            end
        end
        if bingo then return true end
    end
end

function check_win(board)
    return check_rows(board) or check_cols(board)
end

function mark_board(draw, board)
    for _, row in ipairs(board) do
        for _, sq in ipairs(row) do
            if sq[1] == draw then sq[2] = true end
        end
    end
end

function get_winning_board(draws, boards)
    for _, draw in ipairs(draws) do
        for boardnum, board in ipairs(boards) do
            mark_board(draw, board)
            if check_win(board) then return {draw, board} end
        end
    end
end

function sum_unmarked(board)
    local sum = 0
    for _, row in ipairs(board) do
        for _, sq in ipairs(row) do
            if not sq[2] then sum = sum + sq[1] end
        end
    end
    return sum
end

first_winning_board = get_winning_board(read_draws(file), read_boards(file))

print("part1:", first_winning_board[1] * sum_unmarked(first_winning_board[2]))

function get_last_winning_board(draws, boards)
    for _, draw in ipairs(draws) do
        local removes = {}

        for boardnum, board in ipairs(boards) do
            mark_board(draw, board)

            if check_win(board) then
                table.insert(removes, boardnum)
            end
        end

        for i = #removes, 1, -1 do
            if #boards > 1 then
                table.remove(boards, removes[i])
            else
                return {draw, boards[1]}
            end
        end
    end
end

last_winning_board = get_last_winning_board(read_draws(file), read_boards(file))

print("part2:", last_winning_board[1] * sum_unmarked(last_winning_board[2]))
