result = 1
for i in range(2, 21):
    g = result
    while g % i != 0:
        g += result
    result = g

print(result)