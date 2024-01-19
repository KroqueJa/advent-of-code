module Main where
import Lib (readFileStrict)

import Day1

import System.Environment (getArgs)

main :: IO ()
main = do
    args <- getArgs
    case args of
        [filePath] -> do
            contents <- readFileStrict filePath
            (part1, part2) <- return $ solve contents
            putStr "Part 1: "
            print part1
            putStr "Part 2: "
            print part2
        _ -> putStrLn "Usage: day1_hs <file-path>"
