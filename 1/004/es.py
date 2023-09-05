e = []
for i in range(100,1000):
    for g in range(100,1000):
        result = i * g
        re = int(str(result)[::-1])
        if result == re:
            e.append(result)
print(max(e))