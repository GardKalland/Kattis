from sys import stdin
f = stdin
n = int(f.readline())
t = [[int(i) for i in x.split()] for x in f.readlines()]



for x in t:
    if x[2] % x[1] == 0 or x[2] % x[0] == 0:
        print("Yes")
    elif (x[1]+x[0]) % x[2] == 0:
        print("Yes")
    else:
        print("No")