import sys
from typing import Tuple
from math import sqrt, floor

def spiral_step(n: int) -> Tuple[int, int]:
    if n == 0:
        return (0, 0)

    g = int(floor(sqrt(float(n))))
    r = (g + g % 2) // 2
    q = 4 * r**2
    d = n - q

    if n <= (q - 2 * r):
        return (d + 3 * r, r)
    elif n <= q:
        return (r, -d - r)
    elif n <= (q + 2 * r):
        return (r - d, -r)
    else:
        return (-r, d - 3 * r)

def find_first_larger(target: int) -> int:
    grid = dict()
    grid[(0, 0)] = 1

    directions = [(x, y) for x in [-1, 0, 1] for y in [-1, 0, 1]]

    def go(n: int, target: int) -> int:
        x, y = spiral_step(n)
        sum_neighbors = 0
        for dir in directions:
            neighbor = (x + dir[0], y + dir[1])
            if neighbor in grid.keys():
                sum_neighbors += grid[neighbor]

        grid[(x, y)] = sum_neighbors

        if sum_neighbors > target:
            return sum_neighbors
        else:
            return go(n + 1, target)

    return go(1, target)

def main():
    if len(sys.argv) != 2:
        print("Usage: python day3.py [input]")
        exit(0)

    number = int(sys.argv[1])

    # Part 1
    x, y = spiral_step(number-1)
    part1 = abs(x) + abs(y)

    # Part 2
    part2 = find_first_larger(number)

    print(f"Part 1: {part1}\nPart 2: {part2}")

if __name__ == "__main__":
    main()
