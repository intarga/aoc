-- stack --resolver lts script
readInts :: String -> [Int]
readInts = map read . lines

pairDepths :: [Int] -> [(Int, Int)]
pairDepths depths = zip (init depths) $ tail depths

countIncreases = length . filter (\ (a,b) -> a < b)

pairDepths3 :: [Int] -> [(Int, Int)]
pairDepths3 depths = zip depths $ drop 3 depths

main = do
    contents <- readFile "../INPUT/01"
    let part1 = show . countIncreases . pairDepths . readInts $ contents
    let part2 = show . countIncreases . pairDepths3 . readInts $ contents
    print $ "part1: " ++ part1
    print $ "part2: " ++ part2
