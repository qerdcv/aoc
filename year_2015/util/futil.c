#include <stdio.h>
#include <stdlib.h>

FILE* open_file(char* path, char* mode)
{
  FILE* f = fopen(path, mode);
  if (f == NULL)
  {
    printf("ERROR: failed to open file %s\n", path);
    exit(1);
  }

  return f;
}

void close_file(FILE* f) {
  if (fclose(f) < 0) {
    printf("ERROR: failed to close file\n");
    exit(1);
  }
}