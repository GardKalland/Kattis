from sys import stdin



read = int(stdin.readline())
read1 = stdin.readline().split()
read2 = stdin.readline().split()



n = ((read-1) ** 2 + (read-1)) /2

a = int(read1[0]) * read + int(read1[1]) * n

b = int(read2[0]) * read + int(read2[1]) * n

if a < b:
    print("Alice")

elif a > b:
    print("Bob")

else: 
    print("=")
