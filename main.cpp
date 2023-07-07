#include "programs/calc"
#include "programs/solvex"
using namespace std;
string com;
int main() {
  while (com != "exit") {
    printf("mathcmd> ");
    cin >> com;
    if (com == "solvex")
      solvex();
  }
  return 0;
}
