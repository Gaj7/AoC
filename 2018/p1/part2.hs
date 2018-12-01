import Data.Char
import Data.String
import Data.List

toNum :: String -> Int
toNum str = (if (head str == '-') then -1 else 1) * (read (tail str))

-- This is actually quite slow right now. A better solution would be a
-- dictionary/hash map, since I imagine the search is what is so slow
firstRepeat :: [Int] -> Int
firstRepeat list = foo [] 0 list
  where
    foo prev total (x:xs) = let newSum = total + x in
                              if (elem newSum prev) then newSum
                              else foo (newSum:prev) newSum xs

main = do input <- getContents
          print $ firstRepeat $ cycle $ map toNum $ lines input
