#include "wrapper.h"
#include "math_lib.h"
#include <cstring>

int add_number(int a, int b) { return MathLib::add(a, b); }

const char *greet(const char *name) {
  std::string result = MathLib::generateGreeting(name);

  char *c_str = new char[result.length() + 1];
  std::strcpy(c_str, result.c_str());
  return c_str;
}
