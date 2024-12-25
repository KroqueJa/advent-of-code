#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <limits.h>
#include <ctype.h>

#include "lib.h"

inline int parse_int(const char* str, size_t* index) {
    int result = 0;
    while (isdigit(str[*index])) {
        result = result * 10 + (str[*index] - '0');
        (*index)++;
    }
    return result;
}

int main(int argc, char** argv)
{
    if (argc != 2) {
        printf("Usage: day2 [your_input]\n");
        return 0;
    }

    FileInfo info;
    info.contents = malloc(FILE_BUF_SZ);

    read_file(&info, argv[1]);

    // Solution
    size_t part1 = 0;
    size_t part2 = 0;

    // Buffer to memorize numbers
    int32_t parsed[100];    // we assume no more than 100 numbers per line

    // Traverse the bytes once, calculating all necessary information per row
    size_t i = 0;
    size_t j = 0;
    bool found_divisors;
    int32_t imax, imin, enumerator, denominator, current;
    while (i < info.file_sz) {
        // Reset max and min for each line
        imax = INT_MIN;
        imin = INT_MAX;
        enumerator = 0;
        denominator = 0;

        // Reset the parsed numbers iterator;
        j = 0;
        found_divisors = false;

        // Parse numbers until the next newline
        while (i < info.file_sz && info.contents[i] != '\n') {
            current = parse_int(info.contents, &i);
            parsed[j] = current;
            ++j;

            // Update max and min
            if (current > imax) imax = current;
            if (current < imin) imin = current;

            // Check divisors, skip redundant checks
            for (size_t k = 0; k < j - 1; ++k) {
                if (parsed[k] % current == 0) {
                    enumerator = parsed[k];
                    denominator = current;
                    found_divisors = true;
                    break;
                } else if (current % parsed[k] == 0) {
                    enumerator = current;
                    denominator = parsed[k];
                    found_divisors = true;
                    break;
                }
            }

            if (found_divisors) break;

            // Skip whitespace and tabs
            while (i < info.file_sz && (info.contents[i] == ' ' || info.contents[i] == '\t')) {
                i++;
            }
        }
        // Update solutions
        part1 += imax - imin;
        part2 += enumerator / denominator;

        // Skip the newline
        if (i < info.file_sz && info.contents[i] == '\n') {
            i++;
        }
    }

    printf("\nPart 1: %zu\nPart2: %zu\n", part1, part2);

    free(info.contents);
}
