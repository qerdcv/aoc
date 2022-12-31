#include <stdlib.h>

#include "libs/futil.h"
#include "libs/set.h"

typedef struct
{
  int x;
  int y;
} pos;

void update_position(char ch, pos* p) {
  switch (ch)
  {
    case '>':
      p->x++;
      break;
    case '<':
      p->x--;
      break;
    case 'v':
      p->y++;
      break;
    case '^':
      p->y--;
      break;
    default:
      return;
  }
}


int solve_p1() {
  FILE* f = open_file("inputs/day_3.txt", "r");

  Set* s = new_set();

  pos p = {0, 0};

  char buf[1024];
  sprintf(buf, "%d|%d", p.x, p.y);
  insert_into_set(s, buf);
  while (!feof(f)) {
    update_position(getc(f), &p);
    sprintf(buf, "%d|%d", p.x, p.y);
    insert_into_set(s, buf);
  }

  close_file(f);
  int result = s->length;
  free_set(s);
  return result;
}

int solve_p2() {
  FILE* f = open_file("inputs/day_3.txt", "r");

  Set* s = new_set();

  pos santa_pos = {0, 0};
  pos robo_pos = {0, 0};

  char ch;
  char buf[1024];

  sprintf(buf, "%d|%d", 0, 0);
  insert_into_set(s, buf);
  int house_idx = 0;
  while (!feof(f)) {
    house_idx++;
    ch = getc(f);
    if (house_idx % 2 == 0)
      update_position(ch, &robo_pos);
    else
      update_position(ch, &santa_pos);

    if (house_idx % 2 == 0)
      sprintf(buf, "%d|%d", robo_pos.x, robo_pos.y);
    else
      sprintf(buf, "%d|%d", santa_pos.x, santa_pos.y);

    insert_into_set(s, buf);
  }

  close_file(f);
  int result = s->length;
  free_set(s);
  return result;
}

int main()
{
  printf("PART 1: %d\n", solve_p1()); // 2565
  printf("PART 2: %d\n", solve_p2()); // 2639
  return 0;
}
