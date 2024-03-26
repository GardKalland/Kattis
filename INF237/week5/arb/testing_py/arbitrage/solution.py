MAXN = 220


while True:
    n = input().strip()
    if not n or n == "0":
        break
    n = int(n)

    ind = {}
    dis = [[0.0 for _ in range(MAXN)] for _ in range(MAXN)]

    for i in range(n):
        name = input().strip()
        ind[name] = i
        for j in range(n):
            dis[i][j] = 0.0
        dis[i][i] = 1.0

    m = int(input().strip())
    for _ in range(m):
        line = input().strip().split()
        c1, c2, rate = line[0], line[1], line[2].split(':')
        a, b = map(int, rate)
        i1 = ind[c1]
        i2 = ind[c2]
        dis[i1][i2] = max(dis[i1][i2], b / a)

    for k in range(n):
        for i in range(n):
            for j in range(n):
                dis[i][j] = max(dis[i][j], dis[i][k] * dis[k][j])

    flag = True
    for i in range(n):
        if dis[i][i] > 1.0:
            flag = False
            break
    print("Ok" if flag else "Arbitrage")
