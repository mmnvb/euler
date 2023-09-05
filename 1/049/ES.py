def prov(x , y , z):
    m = []
    a = int(x)
    k = 0
    for i in range(2, a // 2 + 1):
        if (a % i == 0):
            k = k + 1
    if (k <= 0):
        m.append(True)

    else:
        m.append(False)
        return any(m)

    a = int(y)
    k = 0
    for i in range(2, a // 2 + 1):
        if (a % i == 0):
            k = k + 1
    if (k <= 0):
        m.append(True)

    else:
        m.append(False)

    a = int(z)
    k = 0
    for i in range(2, a // 2 + 1):
        if (a % i == 0):
            k = k + 1
    if (k <= 0):
        m.append(True)

    else:
        m.append(False)
    return all(m)

def rec(x, f, count=0, n=0):
    s = [x]
    while n < f:
        d = s[count] + 3330
        s.append(d)
        count += 1
        n += 1
    return s

for i in range(999, 3340):
    q = rec(i, 2)

    # r = [1245, 4521, 4135]
    d = list(map(lambda x: str(x), q))
    count = 0
    count2 = 0
    d1 = d[0]
    d2 = d[1]
    d3 = d[2]
    t = prov(d1, d2, d3)
    if t == True:
        count = 0
        count2 = 0
        for i in d[0]:
            if i in d[1]:
                count += 1
            if count == 4:
                for i in d[0]:
                    if i in d[2]:
                        count2 += 1
            if count2 == 4:
                print(d)
                print(str(d1)+str(d2)+str(d3))


#296962999629


