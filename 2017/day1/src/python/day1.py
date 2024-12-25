import sys

def main():
    if len(sys.argv) != 2:
        print("Usage: python day1.py [input]")
        exit(0)

    filename = sys.argv[1]
    with open(filename, "rb") as f:
        contents = f.read().strip()

    sum1 = 0
    sum2 = 0
    l = len(contents)
    halfL = l // 2
    for idx, c in enumerate(contents):
        if contents[(idx+1) % l] == c:
            sum1 += c - 48
        if contents[(idx + halfL) % l] == c:
            sum2 += c - 48

    print(f"Part 1: {sum1}\nPart2: {sum2}")

if __name__ == "__main__":
    main()
