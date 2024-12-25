module Day2 (solve) where

import Data.Maybe
import Data.List (foldr1)

solve :: String -> (Int, Int)
solve input =
    let
        rows = parse input

        -- Part 1
        maxMinOfEach = map fromJust $ map maxMin rows
        diffs = map (\(x, y) -> y - x) maxMinOfEach

        -- Part 2
        divisorPairs = map fromJust $ map findDivisorPair rows
        divisions = map divLoByHi divisorPairs
    in
        (sum diffs, sum divisions)

    where
        parse :: String -> [[Int]]
        parse = map (map read . words) . lines

        maxMin :: (Ord a) => [a] -> Maybe (a, a)
        maxMin lst = if length lst < 2 then Nothing else
            let
                smol = foldr1 min lst
                big = foldr1 max lst
            in
                Just (smol, big)

        findDivisorPair :: (Integral a) => [a] -> Maybe (a, a)
        findDivisorPair [] = Nothing
        findDivisorPair [_] = Nothing
        findDivisorPair (x1:x2:xs) = case filter (\x -> x1 `mod` x == 0 || x `mod` x1 == 0) (x2:xs) of
            [] -> findDivisorPair (x2:xs)
            [divisor] -> Just (x1, divisor)

        divLoByHi :: (Int, Int) -> Int
        divLoByHi (a, b) = if a < b then b `div` a else a `div` b
