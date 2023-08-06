is_palindrome = lambda number: True if (str_n := str(number)) == str_n[::-1] else False
l_max = 0

for i in range(100, 1000):
    for g in range(100, 1000):
        if is_palindrome((x := i * g)) and x > l_max:
            l_max = x

print(l_max)