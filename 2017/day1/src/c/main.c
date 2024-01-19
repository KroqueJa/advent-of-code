#include <stdio.h>

#include "lib.h"

int main(int argc, char** argv)
{
    if (argc != 2) {
        printf("Usage: day1 [your_input]\n");
        return 0;
    }

    FileInfo info;
    info.contents = malloc(FILE_BUF_SZ);

    read_file(&info, argv[1]);

    // Part 1

    size_t sum1 = 0;
    size_t sum2 = 0;
    const size_t arr_sz = info.file_sz - 1;
    const size_t half_sz = arr_sz / 2;

    // Ignore both null terminator and newline
    for (size_t i = 0; i < arr_sz; ++i) {
        if (info.contents[i] == info.contents[i+1]) {
            sum1 += info.contents[i] - '0';
        }
        if (info.contents[i] == info.contents[(i + half_sz) % arr_sz]) {
            sum2 += info.contents[i] - '0';
        }
    }


    // Check once whether the list loops around
    if (info.contents[0] == info.contents[arr_sz-1]) {
        sum1 += info.contents[0] - '0';
    }

    printf("\nPart 1: %zu\nPart2: %zu\n", sum1, sum2);

    free(info.contents);
}
