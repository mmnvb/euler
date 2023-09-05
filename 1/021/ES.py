a = []
for n in range(1, 10000):
    suma = []
    suma2 = []

    for i in range(1, int(n / 2) + 1):
        if n % i == 0:
            suma.append(i)

    for g in range(1, int(sum(suma) / 2) + 1):
        if sum(suma) % g == 0:
            suma2.append(g)

    if sum(suma2) == n:
        if n != sum(suma):
            a.append(n)
            a.append(sum(suma))

print(sum(set(a)))