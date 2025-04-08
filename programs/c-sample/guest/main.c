extern void printsomething(int a);
extern int add(int a, int b);

#include <zkvm.h>

int cmain()
{
  int b = 42; // unused;
  int a = 21;
  printsomething(a);

  int *p = malloc(4 * sizeof(int));
  for (int i = 0; i < 4; i++) {
    p[i] = i;
  }
  for (int i = 0; i < 4; i++) {
    printf("%d ", p[i]);
  }

  free(p);
  printf("Hello, world!\n");
  printf("The answer is %d\n", add(a, a));
  return add(a, a);
}
