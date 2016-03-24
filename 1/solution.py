def sum_3_5(n):
    return sum(x for x in range(n) if x % 3 == 0 or x % 5 == 0)

assert sum_3_5(10) == 23

print(sum_3_5(1000))
