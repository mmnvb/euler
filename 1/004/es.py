e = []
for i in range(600,1000):
    for g in range(600,1000):
        result = i * g
        re = int(str(result)[::-1])
        if result == re:
            s = i, "*" ,g ,"=" ,result
            e.append(s)
print(max(e))
