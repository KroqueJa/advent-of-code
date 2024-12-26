module Main where

import System.Environment (getArgs)
import System.Exit (die)
import Text.Read (readMaybe)

import Day3

main :: IO ()
main = do
    args <- getArgs
    case args of
        [numberStr] -> case readMaybe numberStr of
            Just number -> do
                let (part1, part2) = solve number
                putStrLn $ "Part 1: " ++ show part1
                putStrLn $ "Part 2: " ++ show part2
            Nothing -> die "Error: Argument is not a valid number."
        _ -> die "Usage: program <number>"
