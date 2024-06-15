from sys import stdin
n = int(stdin.readline().strip())
a = [int(x) for x in stdin.readline().split()]
a.sort()

while a:
    if a[-1] >= len(a):
        
        a.pop()
    else:
        break

if a:
    print(max(a) + (n - len(a)))
else:
    print(n)