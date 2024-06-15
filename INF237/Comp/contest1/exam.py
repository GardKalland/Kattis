n = int(input())

corr = 3

identical = 0

for i,j in zip(my_answ, his_answ):
    if i == j:

        identical += 1

        


quest = len(my_answ)

if identical <= corr:
    diff = identical +  quest - corr
    print(diff)

else: 
    diff = corr + quest - identical
    print(diff)
