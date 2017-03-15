import itertools


def is_palindrome(n):
    s = str(n)
    return s == s[::-1]

nums = range(100, 1000)
pairs = itertools.product(nums, nums)

print(max(filter(is_palindrome, map(lambda (x, y): x*y, pairs))))
