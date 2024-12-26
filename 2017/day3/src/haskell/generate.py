import math

def generate_spiral_sequence(size):
    # Initialize grid as a 1D array
    h = 2 * size - 1
    grid = [0 for _ in range(h * h)]
    m = size

    # Directions for neighbors
    directions = [
        [1, 0], [1, -1], [0, -1], [-1, -1],
        [-1, 0], [-1, 1], [0, 1], [1, 1]
    ]

    # Place the first cell in the center
    grid[m * h + m] = 1
    sequence = [1]

    # Generate the sequence
    for n in range(1, (h - 2)**2):
        g = int(math.sqrt(n))
        r = (g + g % 2) // 2
        q = 4 * r**2
        d = n - q

        # Determine grid coordinates (j, k)
        if n <= q - 2 * r:
            j = d + 3 * r
            k = r
        elif n <= q:
            j = r
            k = -d - r
        elif n <= q + 2 * r:
            j = r - d
            k = -r
        else:
            j = -r
            k = d - 3 * r

        j += m
        k += m

        # Calculate the sum of neighbors
        s = 0
        for dx, dy in directions:
            x, y = j + dx, k + dy
            if 0 <= x < h and 0 <= y < h:
                s += grid[x * h + y]

        # Update grid and sequence
        grid[j * h + k] = s
        sequence.append(s)

    return sequence

# Example usage
sequence = generate_spiral_sequence(5)
print(sequence)

