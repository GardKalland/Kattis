

list = [0, 2, 3, 5]

n = int(input())

for i in range(3, 10005):
    list.append(list[i-1] + list[i])

for i in range(n):
    t = int(input())
    print(list[t] % 1000000007)
