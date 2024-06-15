
def triangle_area(x1, y1, x2, y2, x3, y3):
	tmp = abs(x1*(y2-y3)+x2*(y3-y1)+x3*(y1-y2))
	return tmp/2


input_1 = list(map(int, input().split()))
input_2 = list(map(int, input().split()))
input_3 = list(map(int, input().split()))

x1, y1, x2, y2, x3, y3 = input_1[0], input_1[1], input_2[0], input_2[1], input_3[0], input_3[1]
areal = triangle_area(x1, y1, x2, y2, x3, y3)





n = int(input())
count = 0

for i in range(0, n):
    l = list(map(int, input().strip().split()))
    x, y = l[0], l[1]
    if areal == triangle_area(x, y, x2, y2, x3, y3) + triangle_area(x1, y1, x, y, x3, y3) + triangle_area(x1, y1, x2, y2, x, y):
        count += 1

print(areal)
print(count)
