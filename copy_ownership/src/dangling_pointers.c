#include <stdio.h>
#include <stdlib.h>

int main ()
{

	char *s0 = (char *) malloc (sizeof(char) * 5);

	s0[0] = 'a';
	s0[1] = 'a';
	s0[2] = 'a';
	s0[3] = 'a';
	s0[4] = '\0';

	printf("%p %s\n",s0, s0);

	char *s1 = s0;

	printf("%p %s\n",s1, s1);

	free(s0);

	printf("%p %s\n",s1, s1);


	return 0;
}