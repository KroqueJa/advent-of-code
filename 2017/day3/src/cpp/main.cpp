#include <functional>
#include <iostream>
#include <unordered_map>
#include <utility> // std::pair
#include <vector>

using P = std::pair<int32_t, int32_t>;

// Hash function for pair
namespace std {
    template <> struct hash<P> {
        std::size_t operator()(const P &key) const noexcept {
            std::size_t h1 = std::hash<uint64_t>{}(key.first);
            std::size_t h2 = std::hash<uint64_t>{}(key.second);
            return h1 ^ (h2 << 1);
        }
    };
} // namespace std

using Grid = std::unordered_map<P, uint32_t>;

P spiralStep(const uint32_t n) {
    uint32_t g = std::floor(std::sqrt(static_cast<float>(n)));
    uint32_t r = (g + g % 2) / 2;
    uint32_t q = 4 * std::pow(r, 2);
    uint32_t d = n - q;

    if (n <= (q - 2 * r)) {
        return {d + 3 * r, r};
    } else if (n <= q) {
        return {r, -d - r};
    } else if (n <= (q + 2 * r)) {
        return {r - d, -r};
    } else {
        return {-r, d - 3 * r};
    }
}

uint32_t findFirstLarger(const uint32_t target) {
    Grid grid;
    P origin{0, 0};
    grid.emplace(origin, 1);

    const std::vector<P> directions = {
        {-1, -1}, {0, -1}, {1, -1}, {-1, 0}, {1, 0}, {-1, 1}, {0, 1}, {1, 1},
    };

    uint32_t n = 1;

    while (true) {
        P step = spiralStep(n);

        uint32_t sumNeighbors = 0;
        for (const auto& dir : directions) {
            P neighbor = {step.first + dir.first, step.second + dir.second};
            auto neighborIt = grid.find(neighbor);
            if (neighborIt != grid.end()) {
                sumNeighbors += neighborIt->second;
            }
        }

        grid.emplace(step, sumNeighbors);

        if (sumNeighbors > target) {
            return sumNeighbors;
        }

        ++n;
    }
}

int main(int argc, char **argv) {
    if (argc != 2) {
        std::cout << "Usage: day3_cpp [input]" << std::endl;
        return 0;
    }

    uint32_t number = std::stoi(argv[1]);

    // Part 1
    const auto [x, y] = spiralStep(number - 1);
    std::cout << "Part 1: " << std::abs(x) + std::abs(y) << std::endl;
    // Part 2
    std::cout << "Part 2: " << findFirstLarger(number) << std::endl;

    // Prepare a grid
}
