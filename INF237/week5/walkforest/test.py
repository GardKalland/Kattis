import sys
from heapq import heappush, heappop
from collections import defaultdict

def reverse_dijkstra(graph, target):
    distances = {node: float('infinity') for node in graph}
    distances[target] = 0
    pq = [(0, target)]

    while pq:
        current_distance, current_node = heappop(pq)
        if current_distance > distances[current_node]:
            continue

        for neighbor, weight in graph[current_node].items():
            distance = current_distance + weight
            if distance < distances[neighbor]:
                distances[neighbor] = distance
                heappush(pq, (distance, neighbor))

    return distances

def count_paths(graph, start, distances):
    if start == 2:
        return 1

    count = 0
    for neighbor, _ in graph[start].items():

        if distances[neighbor] < distances[start]:
            count += count_paths(graph, neighbor, distances)

    return count

def solve(test_cases):
    results = []
    for graph in test_cases:

        distances = reverse_dijkstra(graph, 2)


        path_count = count_paths(graph, 1, distances)
        results.append(path_count)

    return results


test_cases = []
current_test_case = defaultdict(dict)

for line in sys.stdin:
    line = line.strip()
    if line == "0":
        if current_test_case:
            test_cases.append(current_test_case)
        break

    if ' ' in line:
        parts = list(map(int, line.split()))
        if len(parts) == 2:
            if current_test_case:
                test_cases.append(current_test_case)
                current_test_case = defaultdict(dict)
        else:
            a, b, d = parts
            current_test_case[a][b] = d
            current_test_case[b][a] = d


results = solve(test_cases)


for count in results:
    print(count)
