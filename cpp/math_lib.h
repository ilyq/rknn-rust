#ifndef MATH_LIB_H
#define MATH_LIB_H

#include <string.h>
#include <string>

class MathLib {
public:
  static int add(int a, int b);
  static std::string generateGreeting(const std::string &name);
};

#endif