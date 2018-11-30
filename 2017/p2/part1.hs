import Data.Char
import Data.String

listify :: String -> [[String]]
listify = map words . lines

numbify :: [[String]] -> [[Int]]
numbify = map (map read)

checksum :: [[Int]] -> Int
checksum = sum . (map (\l -> maximum l - (minimum l)))

main = do input <- getContents
          print $ checksum $ numbify $ listify input
