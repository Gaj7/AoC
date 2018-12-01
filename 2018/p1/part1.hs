import Data.Char
import Data.String

toNum :: String -> Int
toNum str = (if (head str == '-') then -1 else 1) * (read (tail str))

main = do input <- getContents
          print $ sum (map toNum (lines input))
