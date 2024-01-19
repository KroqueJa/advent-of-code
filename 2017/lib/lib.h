#include <stdlib.h>
#include <stdio.h>

#define FILE_BUF_SZ 8192

typedef struct FileInfo {
    char* contents;
    size_t file_sz;
} FileInfo;

void read_file(FileInfo* info, const char* path)
{
    FILE* f = fopen(path, "rb");

    if (f == NULL) {
        perror("ERROR | Could not open file");
        exit(1);
    }

    fseek(f, 0L, SEEK_END);
    info->file_sz = ftell(f);
    rewind(f);
    size_t read_sz = fread(info->contents, 1, info->file_sz, f);
    if (read_sz != info->file_sz) {
        perror("ERROR | Could not read the entire file");
        fclose(f);
    }
    fclose(f);
}

