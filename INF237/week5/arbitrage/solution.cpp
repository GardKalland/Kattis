#include <algorithm>
#include <iostream>
#include <limits>
#include <map>
#include <string>
#include <vector>
using namespace std;

const int MAXN = 200;
double dis[MAXN][MAXN];

void resetGraph(int n) {
  for (int i = 0; i < n; i++) {
    for (int j = 0; j < n; j++) {
      dis[i][j] = (i == j) ? 1.0 : numeric_limits<double>::min();
    }
  }
}

void floydWarshall(int n, bool &hasArbitrage) {
  for (int k = 0; k < n; ++k) {
    for (int i = 0; i < n; ++i) {
      for (int j = 0; j < n; ++j) {
        if (dis[i][k] * dis[k][j] > dis[i][j]) {
          dis[i][j] = dis[i][k] * dis[k][j];
          if (i == j && dis[i][j] > 1.0) {
            hasArbitrage = true;
            return;
          }
        }
      }
    }
  }
}

int main() {
  int n, m;
  while (cin >> n && n) {
    map<string, int> currencyIndex;
    string currency, from, to;
    for (int i = 0; i < n; ++i) {
      cin >> currency;
      currencyIndex[currency] = i;
    }

    resetGraph(n);

    cin >> m;
    for (int i = 0; i < m; ++i) {
      int rateFrom, rateTo;
      char separator;
      cin >> from >> to >> rateFrom >> separator >> rateTo;
      int idxFrom = currencyIndex[from], idxTo = currencyIndex[to];
      dis[idxFrom][idxTo] = max(dis[idxFrom][idxTo], (double)rateTo / rateFrom);
    }

    bool hasArbitrage = false;
    floydWarshall(n, hasArbitrage);

    cout << (hasArbitrage ? "Arbitrage" : "Ok") << endl;
  }
  return 0;
}
