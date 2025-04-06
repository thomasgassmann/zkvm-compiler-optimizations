extern void printsomething(int a);
extern int add(int a, int b);
extern int printf(const char *fmt, ...);

int cmain()
{
  int b = 42; // unused;
  int a = 21;
  printsomething(a);
  printf("Hello, world!\n");
  printf("Hello, world! formatted: %d\n", a);
  return add(a, a);
}
