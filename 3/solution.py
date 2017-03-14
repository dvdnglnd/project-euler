# Implementation of Pollard's rho
import math
import collections


def is_prime(n):
    if n % 2 == 0 and n > 2:
        return False
    return all(n % i for i in range(3, int(math.sqrt(n)) + 1, 2))


def g_n(n):
    def g(x):
        return (x**2 + 1) % n
    return g


def gcd(a, b):
    if b == 0:
        return a
    else:
        return gcd(b, a % b)


def rho(n):
    x = 2
    y = 2
    d = 1
    g = g_n(n)
    while d == 1:
        x = g(x)
        y = g(g(y))
        d = gcd(abs(x - y), n)
    if d == n:
        return None
    else:
        return d


def factors(n):
    if n == 1:
        return
    elif is_prime(n):
        yield n
    else:
        divisor = rho(n)
        yield factors(divisor)
        yield factors(n / divisor)


def flatten(it):
    for x in it:
        if (isinstance(x, collections.Iterable) and
            not isinstance(x, str)):
            for y in flatten(x):
                yield y
        else:
            yield x


assert(max(flatten(factors(13195))) == 29)

print(max(flatten(factors(600851475143))))
