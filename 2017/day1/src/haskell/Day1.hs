module Day1 (solve) where

import Data.Char (digitToInt)
import Data.Array

solve :: String -> (Int, Int)
solve input =
    let
        noNewline = init input                              -- Remove the newline character
        listSize = length noNewline                        -- Number of digits
        digits = listArray (1, listSize) (map digitToInt noNewline) -- Create 1-based array
        halfLen = listSize `div` 2                         -- Halfway point for part 2
    in
        (sumMatchesNext (1, listSize) digits,              -- Part 1: Match next (wrapping around)
         sumMatchesHalf (1, listSize) digits halfLen)      -- Part 2: Match halfway

    where
        sumMatchesNext :: (Int, Int) -> Array Int Int -> Int
        sumMatchesNext bounds@(low, high) arr =
            let
                pairs = [(arr ! i, arr ! ((i `mod` high) + 1)) | i <- range bounds]
                matchNext = filter (\(a, b) -> a == b) pairs
            in
                sum $ map fst matchNext

        sumMatchesHalf :: (Int, Int) -> Array Int Int -> Int -> Int
        sumMatchesHalf bounds@(low, high) arr halfLen =
            let
                pairs = [(arr ! i, arr ! ((i + halfLen - 1) `mod` high + 1)) | i <- range bounds]
                matchHalf = filter (\(a, b) -> a == b) pairs
            in
                sum $ map fst matchHalf
