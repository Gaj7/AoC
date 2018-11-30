import Data.Char
import Data.String

listify :: String -> [[String]]
listify = map words . lines

numbify :: [[String]] -> [[Int]]
numbify = map (map read)

checksum :: [[Int]] -> Int -- Quick and dirty solution, but it worked first try
checksum = sum . (map (\row -> head [div a b | a <- row, b <- row, a > b, mod a b == 0]))

main = do input <- getContents
          print $ checksum $ numbify $ listify input
