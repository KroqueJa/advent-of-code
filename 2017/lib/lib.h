#include <stdlib.h>
#include <stdio.h>

#define FILE_BUF_SZ 8192

typedef struct FileInfo {
    char* contents;
    size_t file_sz;
} FileInfo;

void read_file(FileInfo* info, const char* path);
