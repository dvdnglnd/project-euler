/**
  Solution to project Eurler #1
  */
def sum3Or5(n: Int): Int = {
  (1 until n).filter(x => x%3 == 0 || x%5 == 0).sum
}

assert(sum3Or5(10) == 23)

println(sum3Or5(1000))
