#include <stdio.h>
#include <stdbool.h>

#include "libs/futil.h"

#define GRID_SIZE 1000
#define BUF_SIZE 100

bool grid[GRID_SIZE][GRID_SIZE];

int main()
{
  size_t read = 0;
  FILE* f = open_file("inputs/day_6.txt", "r");
  char buf[BUF_SIZE];

  while ((read = fread(buf, 1, BUF_SIZE, f)) != 0) {
    for (int i = 0; i < read; i++) {
      if (buf[i] == '\n') {
        break;
      }
    }
    memset(buf, 0, BUF_SIZE);
  }
  close_file(f);
  return 0;
}