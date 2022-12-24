#include <stdio.h>

#include "util/futil.h"

int main()
{
  FILE* f = open_file("./inputs/day_1.txt", "r");
  
  if (f == NULL)
  {
    printf("file is null");
  }

  int result_p1 = 0;
  int result_p2 = 0;
  int idx = 0;

  while (!feof(f))
  {
    idx++;
    switch (getc(f))
    {
    case ')':
      result_p1--;
      break;
    case '(':
      result_p1++;
      break;
    }

    if (result_p1 < 0 && result_p2 == 0) {
      result_p2 = idx;
    }
  }

  close_file(f);

  printf("RESULT P1: %d\n", result_p1);
  printf("RESULT P2: %d\n", result_p2);

  return 0;
}