#include <iostream>
#include <limits>
#include <queue>
#include <unordered_map>
#include <unordered_set>
#include <vector>
using namespace std;

typedef pair<int, int> xy;
const int INF = numeric_limits<int>::max();

vector<vector<xy>> graph;
vector<int> dist;
int N, M;

void dijkstra(int src) {
  priority_queue<xy, vector<xy>, greater<xy>> pq;
  dist.assign(N, INF);
  dist[src] = 0;
  pq.push(xy(0, src));

  while (!pq.empty()) {
    int from = pq.top().second;
    pq.pop();

    for (auto &edge : graph[from]) {
      int to = edge.second, weight = edge.first;
      if (dist[to] > dist[from] + weight) {
        dist[to] = dist[from] + weight;
        pq.push(xy(dist[to], to));
      }
    }
  }
}

int countPaths(int currentNode, vector<int> &pathCounts) {
  if (pathCounts[currentNode] != -1) {
    return pathCounts[currentNode];
  }

  int numberOfWays = 0;

  for (const auto &edge : graph[currentNode]) {
    int neighborNode = edge.second;
    int pathWeight = edge.first;

    if (dist[neighborNode] > dist[currentNode]) {
      numberOfWays += countPaths(neighborNode, pathCounts);
    }
  }

  pathCounts[currentNode] = numberOfWays;
  return numberOfWays;
}

int main() {
  while (true) {
    cin >> N >> M;
    if (N == 0)
      break;

    graph.assign(N, vector<xy>());
    dist.assign(N, INF);

    for (int i = 0; i < M; ++i) {
      int a, b, d;
      cin >> a >> b >> d;
      graph[a - 1].push_back(xy(d, b - 1));
      graph[b - 1].push_back(xy(d, a - 1));
    }

    dijkstra(1);

    vector<int> pathCount(N, -1);
    pathCount[0] = 1;

    int totalPaths = countPaths(1, pathCount);

    cout << totalPaths << endl;
    graph.clear();
  }

  return 0;
}
