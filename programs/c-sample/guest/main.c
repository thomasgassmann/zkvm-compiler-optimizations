extern void printsomething(int a);
extern int add(int a, int b);

#include <zkvm.h>

int cmain()
{
  int b = 42; // unused;
  int a = 21;
  printsomething(a);
  printf("Hello, world!\n");
  printf("The answer is %d\n", add(a, a));
  return add(a, a);
}
