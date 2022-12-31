#include <stdio.h>
#include <string.h>

#include "libs/md5.h"
#include "libs/futil.h"

int calc_zeros(char *hash) {
	int zero_cnt = 0;

	for (int i = 0; i < strlen(hash); i++) {
		if (hash[i] == '0')
			zero_cnt++;
		else
			return zero_cnt;
	}

	return zero_cnt;
}

int solve_p1(char* input) {
	char buf[1024];
	int cnt = 1;
	while (1)
	{
		memset(buf, 0, 1024);
		sprintf(buf, "%s%d", input, cnt);
		char* hash = generate_md5_hash(buf);
		if (calc_zeros(hash) == 5)
		{
			free(hash);
			break;
		}

		free(hash);
		cnt++;
	}

	return cnt;
}

int solve_p2(char* input) {
	char buf[1024];
	int cnt = 1;
	while (1)
	{
		memset(buf, 0, 1024);
		sprintf(buf, "%s%d", input, cnt);
		char* hash = generate_md5_hash(buf);
		if (calc_zeros(hash) == 6)
		{
			free(hash);
			break;
		}

		free(hash);
		cnt++;
	}

	return cnt;
}

int main() {
	char* input = "ckczppom";
	printf("PART 1: %d\n", solve_p1(input));
	printf("PART 2: %d\n", solve_p2(input));
	return 0;
}
