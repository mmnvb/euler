from math import sqrt
for i in range(1,500,2):
    i = i**2
    for j in range(1,500,):
        j = j**2
        s = sqrt(i) + sqrt(j) + sqrt(i+j)
        if s == 1000:
            print(sqrt(i),"*",sqrt(j),"*",sqrt(i+j),"=",sqrt(i) * sqrt(j) * sqrt(i+j) )
