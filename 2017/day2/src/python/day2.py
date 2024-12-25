import sys

def main():
    if len(sys.argv) != 2:
        print("Usage: python day2.py [input]")
        exit(0)

    filename = sys.argv[1]

    part1 = 0
    part2 = 0
    with open(filename, "rb") as f:
        for line in f:
            numbers = [int(word) for word in line.split()]
            imin = int(1000000) # lazy - just a big number
            imax = 0

            enum = 0
            denom = 0
            for number in numbers:
                # Find min and max
                if number < imin:
                    imin = number
                if number > imax:
                    imax = number

            # Find divisors
            for idx, number in enumerate(numbers):
                for j in range(idx, len(numbers)):
                    if numbers[idx] % numbers[j] == 0:
                        enum = max(numbers[idx], numbers[j])
                        denom = min(numbers[idx], numbers[j])

            part1 += imax - imin
            part2 += enum // denom

    print(f"Part 1: {part1}\nPart2: {part2}")

if __name__ == "__main__":
    main()
