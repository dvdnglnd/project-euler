def isPrime(n: Long): Boolean = {
  if (n % 2 == 0 && n > 2) false
  else {
    val xs = for (x <- 3 to Math.sqrt(n).toInt + 1 by 2) yield n % x
    xs.forall(_ != 0)
  }
}

def largestFactor(n: Long): Long = {
  (for (x <- 2 to Math.sqrt(n).toInt + 1 if isPrime(x) && n % x == 0)yield x).reduce(_ max _)
}

assert(largestFactor(13195) == 29)

print(largestFactor(600851475143L))
