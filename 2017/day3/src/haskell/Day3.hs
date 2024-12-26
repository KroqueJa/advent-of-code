module Day3 where

import Control.Monad (forM_)
import Control.Monad.State
import Data.List (genericIndex)
import Data.Map (Map)
import Data.Maybe (fromMaybe)
import qualified Data.Map as Map

type Grid = Map.Map (Int, Int) Int


-- This solution owes greatly to https://oeis.org/A141481

solve :: Int -> (Int, Int)
solve i = (solvePart1 i, solvePart2 i)
    where
        -- Part 1: Calculate the Manhattan distance
        solvePart1 :: Int -> Int
        solvePart1 i =
            let (x, y) = spiralStep (i - 1) -- Convert to zero-based index
            in abs x + abs y

        -- Part 2: Find the first value larger than the input
        solvePart2 :: Int -> Int
        solvePart2 = findFirstLarger

        -- Spiral directions
        directions :: [(Int, Int)]
        directions = [ (1, 0), (1, -1), (0, -1), (-1, -1)
                     , (-1, 0), (-1, 1), (0, 1), (1, 1)]

        -- Compute the coordinates for the nth position in the spiral
        spiralStep :: Int -> (Int, Int)
        spiralStep n =
            let g = floor (sqrt (fromIntegral n))
                r = (g + g `mod` 2) `div` 2
                q = 4 * r ^ 2
                d = n - q
            in if n <= q - 2 * r then (d + 3 * r, r)
               else if n <= q then (r, -d - r)
               else if n <= q + 2 * r then (r - d, -r)
               else (-r, d - 3 * r)

        -- Find the first value larger than the target
        findFirstLarger :: Int -> Int
        findFirstLarger target = go 1 (Map.singleton (0, 0) 1)
          where
            go :: Int -> Grid -> Int
            go n grid =
                let (x, y) = spiralStep n

                    -- Calculate the sum of neighbors
                    neighbors = [(x + dx, y + dy) | (dx, dy) <- directions]
                    sumNeighbors = sum $ map (\coord -> Map.findWithDefault 0 coord grid) neighbors

                    -- Update the grid with the new value
                    newGrid = Map.insert (x, y) sumNeighbors grid

                in if sumNeighbors > target
                   then sumNeighbors
                   else go (n + 1) newGrid
