from sys import stdin

read = stdin.readline().split()
if int(read[1]) == 1: 
    print("YES")

elif int(read[1]) == 0:
    print("NO")

elif int(read[1]) % 2 == 0:
    if (int(read[0])-1)*2 > int(read[1]):
        print("NO")
    elif (int(read[0])-1) * 2 == int(read[1]):
        print("YES")
    elif (int(read[0])-1) * 2 < int(read[1]):
        print('MAYBE')


else :
    print("MAYBE")


