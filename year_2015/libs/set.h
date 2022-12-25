#include <stdlib.h>
#include <stdbool.h>
#include <string.h>
#include <stdint.h>
#include <stdlib.h>

#include "set.c"

Set* new_set();

bool is_set_empty(Set* set);
bool is_in_set(Set* set, char* member);

void insert_into_set(Set* set, char* member);
void print_set(Set* set);
void free_set(Set* set);