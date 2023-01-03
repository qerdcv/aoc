#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#include "libs/futil.h"

#define LINE_LENGTH 17
#define VOWELS_COUNT 5

char vowels[VOWELS_COUNT] = {'a', 'e', 'i', 'o', 'u'};
char* restricted_substrings[4] = {"ab", "cd", "pq", "xy"};

bool it_has_three_vowels(char* line)
{
  int vowel_cnt = 0;
  for (int i = 0; i < LINE_LENGTH; i++)
  {
    for (int j = 0; j < VOWELS_COUNT; j++)
    {
      if (vowels[j] == line[i]) {
        vowel_cnt++;
      }
    }

    if (vowel_cnt == 3)
    {
      return true;
    }
  }

  return false;
}

bool is_doubled_char(char* line)
{

  for (int i = 0; i < LINE_LENGTH - 1; i++)
  {
    if (line[i] == line[i + 1])
    {
      return true;
    }
  }

  return false;
}

bool is_contains_restricted_substr(char* line)
{

  for (int i = 0; i < LINE_LENGTH - 2; i++)
  {
    char substr[3] = {line[i], line[i + 1], '\0'};
    for (int j = 0; j < 4; j++)
    {
      if (strcmp(substr, restricted_substrings[j]) == 0)
      {
        return true;
      }
    }
  }

  return false;
}

bool is_nice_string(char* line)
{
  return is_doubled_char(line) && it_has_three_vowels(line) && !is_contains_restricted_substr(line);
}

int solve_p1()
{
  FILE* f = open_file("inputs/day_5.txt", "r");
  char line[LINE_LENGTH];
  int nice_str_cnt = 0;

  while (fgets(line, LINE_LENGTH, f))
  {
    getc(f);
    if (is_nice_string(line))
    {
      nice_str_cnt++;;
    }
  }

  close_file(f);
  return nice_str_cnt;
}

bool is_pair_that_appears_twice(char* line)
{
  for (int i = 0; i < LINE_LENGTH - 4; i++)
  {
    char p1[3] = {line[i], line[i + 1], '\0'};
    for (int j = i + 1; j < LINE_LENGTH - 2; j ++)
    {
      char p2[3] = {line[j], line[j + 1], '\0'};
      if (strcmp(p1, p2) == 0)
      {
        if (j == i + 1) // check for overlaps
          return false;

        return true;
      }
    }
  }

  return false;
}

bool is_letter_appears_twice(char* line) {
  for (int i = 0; i < LINE_LENGTH - 2; i++) {
    if (line[i] == line[i + 2])
      return true;
  }

  return false;
}

int solve_p2()
{
  FILE* f = open_file("inputs/day_5.txt", "r");
  char line[LINE_LENGTH];
  int nice_str_cnt = 0;

  while (fgets(line, LINE_LENGTH, f))
  {
    getc(f);
#ifdef __MACH__
    getc(f);
#endif
    printf("%s - line\n", line);
    if (is_pair_that_appears_twice(line) && is_letter_appears_twice(line))
      nice_str_cnt++;
  }

  close_file(f);
  return nice_str_cnt;
}

int main()
{
  printf("PART 1: %d\n", solve_p1());
  printf("PART 2: %d\n", solve_p2()); // 51
  return 0;
}