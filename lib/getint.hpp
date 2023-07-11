#include "strint.hpp"
using namespace std;
inline int getint(string str) {
  string s1;
  for (auto c : str) {
    if (c < '0' || c > '9')
      break;
    s1 = s1 + c;
  }
  return strint(s1);
}
