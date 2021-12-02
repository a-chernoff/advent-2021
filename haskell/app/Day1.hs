module Main where

import System.IO

import Lib

main :: IO ()
main = do
    input <- readInput "../input/1"
    putStrLn $ "part 1: " ++ show (part1 input)
    putStrLn $ "part 2: " ++ show (part2 input)

readInput :: FilePath -> IO [Int]
readInput path = map read . lines <$> readFile path

window :: Int -> [a] -> [[a]]
window _ [] = []
window sz l@(_:ls) = 
    let (n, l') = helper sz l
    in case n of
        0 -> l' : window sz ls
        _ -> []
    where
        helper :: Int -> [a] -> (Int, [a])
        helper 0 ls = (0, [])
        helper n [] = (n, [])
        helper n (l:ls) =
            let (n', ls') = helper (n-1) ls
            in (n', l:ls')

listLessThan :: Ord a => [a] -> Bool
listLessThan [a, b] = a < b

part1 :: [Int] -> Int
part1 = length . filter listLessThan . window 2

part2 :: [Int] -> Int
part2 = part1 . map sum . window 3