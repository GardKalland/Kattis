#include <iostream>
#include <limits>
#include <queue>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

typedef pair<int, int> Edge;
constexpr int MAX_DIST = numeric_limits<int>::max();

vector<vector<Edge>> adjacencyList;
vector<int> shortestDistances;
int nodesCount, edgesCount;

void Dijsktra(int startNode) {
  priority_queue<Edge, vector<Edge>, greater<Edge>> nodesQueue;
  shortestDistances.assign(nodesCount, MAX_DIST);
  shortestDistances[startNode] = 0;
  nodesQueue.emplace(0, startNode);

  while (!nodesQueue.empty()) {
    auto [cost, currentNode] = nodesQueue.top();
    nodesQueue.pop();

    for (const auto &[edgeCost, nextNode] : adjacencyList[currentNode]) {
      if (shortestDistances[nextNode] >
          shortestDistances[currentNode] + edgeCost) {
        shortestDistances[nextNode] = shortestDistances[currentNode] + edgeCost;
        nodesQueue.emplace(shortestDistances[nextNode], nextNode);
      }
    }
  }
}

int calculatePaths(int node, vector<int> &paths) {
  if (paths[node] != -1)
    return paths[node];

  int pathSum = 0;
  for (const auto &[edgeCost, nextNode] : adjacencyList[node]) {
    if (shortestDistances[nextNode] > shortestDistances[node]) {
      pathSum += calculatePaths(nextNode, paths);
    }
  }

  paths[node] = pathSum;
  return pathSum;
}

int main() {
  while (true) {
    cin >> nodesCount >> edgesCount;
    if (nodesCount == 0)
      break;

    adjacencyList.assign(nodesCount, vector<Edge>());
    shortestDistances.assign(nodesCount, MAX_DIST);

    for (int i = 0; i < edgesCount; ++i) {
      int node1, node2, distance;
      cin >> node1 >> node2 >> distance;
      adjacencyList[node1 - 1].emplace_back(distance, node2 - 1);
      adjacencyList[node2 - 1].emplace_back(distance, node1 - 1);
    }

    Dijsktra(1);

    vector<int> pathCounts(nodesCount, -1);
    pathCounts[0] = 1;

    int totalPaths = calculatePaths(1, pathCounts);

    cout << totalPaths << endl;
  }

  return 0;
}
