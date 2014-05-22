/* build and run with, e.g.,
   gcc -Wall rlhvals.c && ./a.out 12345 abc32 invalid
   arg[1] "12345" 5-digit? yes, 5-alphanum? yes
   arg[2] "abc32" 5-digit? no, 5-alphanum? yes
   arg[3] "invalid" 5-digit? no, 5-alphanum? no
*/

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int
is_5digit(char *s)
{
	char *end;

	strtol(s, &end, 10);
	return end - s == 5;
}

#define DECIMAL "0123456789"
#define UCASE "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
#define LCASE "abcdefghijklmnopqrstuvwxyz"

static int
is_5alphanum(char *s)
{
	return strspn(s, DECIMAL UCASE LCASE) == 5;
}

int main(int argc, char *argv[])
{
	int i;
	char *p;

	for (i = 1; i < argc; ++i) {
		p = argv[i];
		printf("arg[%d] \"%s\" 5-digit? %s, 5-alphanum? %s\n",
		       i, p,
		       is_5digit(p) ? "yes" : "no",
		       is_5alphanum(p) ? "yes" : "no");
	}
	return 0;
}
