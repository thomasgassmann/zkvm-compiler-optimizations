extern void printsomething(int a);
extern int add(int a, int b);

int cmain()
{
  int b = 42; // unused;
  int a = 21;
  printsomething(a);
  return add(a, a);
}
