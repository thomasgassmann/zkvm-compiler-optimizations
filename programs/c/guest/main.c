#include "bindings.h"

int cmain() {
  printsomething();
  int b = 42; // unused;
  int a = 21;
  return add(a, a);
}
