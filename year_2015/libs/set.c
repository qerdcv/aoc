typedef struct
{
  char** members;
  int length;
  int cap;
} Set;

Set* new_set()
{
  Set* new_set = (Set*)malloc(sizeof(Set));
  new_set->length = 0;
  new_set->cap = 10;
  new_set->members = (char**)malloc(sizeof(char*) * new_set->cap);


  return new_set;
}

bool is_set_empty(Set* set)
{
  return set->length == 0;
}

bool is_in_set(Set* set, char* member)
{
  for (int i = 0; i < set->length; i++)
    if (strcmp(set->members[i], member) == 0)
      return true;

  return false;
}

void insert_into_set(Set* set, char* member)
{
  if (is_in_set(set, member))
    return;
  
  if (set->length + 1 >= set->cap)
  {
    set->cap = set->cap * 2;
    set->members = realloc(set->members, sizeof(char*) * set->cap);
    if (set->members == NULL)
    {
      printf("ERROR: failed to realloc\n");
      exit(1);
    }
  }

  set->members[set->length] = malloc(sizeof(char) * (strlen(member) + 1));
  strcpy(set->members[set->length], member);
  set->length++;
}

void print_set(Set* set)
{
  for (int i = 0; i < set->length; i++)
    if (i == (set->length - 1))
      printf("%s\n", set->members[i]);
    else
      printf("%s, ", set->members[i]);
}

void free_set(Set* set)
{
  for (int i = 0; i < set->length; i++)
    free(set->members[i]);

  free(set->members);
  free(set);
}