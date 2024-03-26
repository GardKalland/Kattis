#include <bits/stdc++.h>
using namespace std;

typedef long long ll;
const ll inf = 1e18 + 10;

vector<ll> reverse_dijkstra(int n, const vector<vector<pair<int, ll>>> &graph,
                            int target) {
  vector<ll> distance(n + 1, inf);
  distance[target] = 0;
  priority_queue<pair<ll, int>, vector<pair<ll, int>>, greater<>> pq;
  pq.push({0, target});

  while (!pq.empty()) {
    ll dist = pq.top().first;
    int node = pq.top().second;
    pq.pop();

    if (dist > distance[node])
      continue;

    for (auto &edge : graph[node]) {
      ll edgeDist = edge.second;
      int nextNode = edge.first;
      if (distance[node] + edgeDist < distance[nextNode]) {
        distance[nextNode] = distance[node] + edgeDist;
        pq.push({distance[nextNode], nextNode});
      }
    }
  }

  return distance;
}

int main() {
  ios::sync_with_stdio(false);
  cin.tie(nullptr);

  int n, m;
  while (cin >> n >> m && n) {
    vector<vector<pair<int, ll>>> graph(n + 1);
    for (int i = 0; i < m; ++i) {
      int u, v;
      ll w;
      cin >> u >> v >> w;
      graph[u].emplace_back(v, w);
      graph[v].emplace_back(u, w);
    }

    vector<ll> distance = reverse_dijkstra(n, graph, 2);
    vector<ll> ways(n + 1, 0);
    ways[2] = 1;

    vector<int> nodes(n);
    iota(nodes.begin(), nodes.end(), 1);
    sort(nodes.begin(), nodes.end(),
         [&distance](int a, int b) { return distance[a] < distance[b]; });

    for (int node : nodes) {
      for (auto &edge : graph[node]) {
        int nextNode = edge.first;
        ll edgeDist = edge.second;
        if (distance[node] + edgeDist == distance[nextNode]) {
          ways[nextNode] += ways[node];
        }
      }
    }

    cout << ways[1] << '\n';
  }

  return 0;
}
