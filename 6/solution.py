nums = range(1, 101)

sum_squares = sum(x**2 for x in nums)

square_sums = sum(nums)**2

print(square_sums - sum_squares)
