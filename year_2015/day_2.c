#include <stdio.h>

#include "util/futil.h"

#define BUF_SIZE 3

char buf[BUF_SIZE];
typedef struct {
  int l;
  int w;
  int h;
} box;

int calc_box(box b)
{
  int lw = b.l * b.w;
  int wh = b.w * b.h;
  int hl = b.h * b.l;

  int result = 2*lw + 2*wh + 2*hl;

  if (lw < wh && lw < hl)
  {
    result += lw;
  } else if (wh < hl)
  {
    result += wh;
  } else {
    result += hl;
  }

  return result;
}

int calc_ribbon_size(box b) {
  int smallest_sides[2] = {INT_MAX, INT_MAX};

  if (b.l <= b.w && b.l <= b.h)
  {
    smallest_sides[0] = b.l;
  } else if (b.l <= b.w || b.l <= b.h)
  {
    smallest_sides[1] = b.l;
  }

  if (b.w <= b.h && b.w <= b.l)
  {
    smallest_sides[0] = b.w;
  } else if (b.w <= b.h || b.w <= b.l)
  {
    smallest_sides[1] = b.w;
  }


  if (b.h <= b.l && b.h <= b.w)
  {
    smallest_sides[0] = b.h;
  } else if (b.h <= b.l || b.h <= b.w)
  {
    smallest_sides[1] = b.h;
  }

  if (smallest_sides[0] == INT_MAX)
  {
    smallest_sides[0] = smallest_sides[1];
  } else if (smallest_sides[1] == INT_MAX)
  {
    smallest_sides[1] = smallest_sides[0];
  }

  return (2 * smallest_sides[0] + 2 * smallest_sides[1]) +
    (b.l * b.w * b.h);
}

box new_box_from_file(FILE* f)
{
  box b = {
    0, 0, 0
  };

  char ch;
  int ch_idx = 0;

  while(!feof(f))
  {
    ch = getc(f);
    switch (ch)
    {
    case '\n': // end of line
      b.h = atoi(buf);
      memset(buf, 0, 3);
      return b;
    break;
    case 'x':
      int val = atoi(buf);
      if (b.l == 0)
      {
        b.l = val;
      } else if (b.w == 0)
      {
        b.w = val;
      }
      memset(buf, 0, 3);
      ch_idx = 0;
      break;
    default:
      buf[ch_idx] = ch;
      ch_idx++;
      break;
    }
  }

  b.h = atoi(buf);
  return b;
}

// 1606483
int solve_p1() {
  FILE* f = open_file("inputs/day_2.txt", "r");
  int result = 0;

  while (!feof(f))
  {
    result += calc_box(new_box_from_file(f));
  }
  close_file(f);
  return result;
}

// !6494341
int solve_p2() {
  FILE* f = open_file("inputs/day_2.txt", "r");

  int result = 0;
  while (!feof(f))
  {
    result += calc_ribbon_size(new_box_from_file(f));
  }

  close_file(f);
  return result;
}

int main()
{
  printf("PART 1: %d\n", solve_p1());
  printf("PART 2: %d\n", solve_p2());

  char ch[3] = {'2', 0, 0};

  printf("%d\n", atoi(ch));
  return 0;
}