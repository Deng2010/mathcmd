#include "../lib/getint.hpp"
#include <iostream>
#include <string>
using namespace std;
double dx, d1;
inline void leftx() {
  int num;
  string s;
  cin >> s;
  num = getint(s);
  if (s[s.size() - 1] == 'x')
    dx += num;
  else
    d1 -= num;
}
inline void rightx() {
  int num;
  string s;
  cin >> s;
  num = getint(s);
  if (s[s.size() - 1] == 'x')
    dx -= num;
  else
    d1 += num;
}
inline void solvex() {
  string opt;
  while (opt != "exit") {
    printf("mathcmd->solvex> ");
    cin >> opt;
    if (opt == "left") {
      leftx();
    } else if (opt == "right") {
      rightx();
    } else if (opt == "end") {
      cout << d1 / dx << endl;
      dx = d1 = 0;
    }
  }
}
