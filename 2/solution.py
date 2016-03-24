def fibonacci(n):
    a, b = 1, 1
    while b < n:
        yield b
        a, b = b, a + b

print(sum(x for x in fibonacci(4000000) if x % 2 == 0))
