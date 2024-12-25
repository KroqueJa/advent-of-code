#include <stdlib.h>
#include <stdio.h>

#define FILE_BUF_SZ 8192

#define MAX(x, y) (((x) > (y)) ? (x) : (y))
#define MIN(x, y) (((x) < (y)) ? (x) : (y))

typedef struct FileInfo {
    char* contents;
    size_t file_sz;
} FileInfo;

void read_file(FileInfo* info, const char* path);
