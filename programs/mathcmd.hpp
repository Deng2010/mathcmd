#include "solvex.hpp"
using namespace std;
double a, b;
string _com;
inline int mathcmd() {
  do {
    cout << "mathcmd> ";
    cin >> _com;
    if (_com == "add") {
      cin >> a >> b;
      cout << a + b << endl;
    } else if (_com == "sub") {
      cin >> a >> b;
      cout << a - b << endl;
    } else if (_com == "mul") {
      cin >> a >> b;
      cout << a * b << endl;
    } else if (_com == "div") {
      cin >> a >> b;
      cout << a / b << endl;
    } else if (_com == "solvex") {
      solvex();
    }
  } while (_com != "exit");
  return 0;
}
