from itertools import islice

def primes():
    n = 2
    found = {}
    while True:
        if n not in found:
            yield n
            found[n * n] = [n]
        else:
            for p in found[n]:
                found.setdefault(p + n, []).append(p)
            del found[n]
        n += 1

print(next(islice(primes(), 10000, 10001)))
