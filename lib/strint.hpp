#include <string>
using namespace std;
inline int strint(string str) {
  int k;
  for (auto c : str) {
    k = k * 10 + c - '0';
  }
  return k;
}
