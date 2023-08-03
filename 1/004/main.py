def is_palindrome(num):
    digit = []
    n = 0
    temp = 0

    while num != 0:
        temp = num % 10
        digit.append(temp)
        num //= 10
        n += 1

    for i in range(0, n):
        if digit[i] != digit[n-i-1]:
            return False

    return True


result = 0
run = True

MIN=100
MAX=1000

for i in reversed(range(MIN, MAX)):
    for j in reversed(range(MIN, MAX)):
        result = i * j
        if is_palindrome(result):
            print(f"{i} x {j} = {result}")
            break
    else:
        continue
    break