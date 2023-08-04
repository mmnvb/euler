sumsquares = 0
squaresum = 0
for i in range(1,101):
    sumsquares +=i**2
for h in range(1,101):
    squaresum +=h
print((squaresum**2)-sumsquares)