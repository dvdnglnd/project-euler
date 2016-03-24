def fibonacci(n: Int): Stream[Int] = {
  def fibs(a: Int = 0, b: Int = 1): Stream[Int] = Stream.cons(a, fibs(b, a+b))

  fibs() takeWhile(_ <= n)
}
println(fibonacci(3000) take 10 toList)
println(fibonacci(4000000).filter(_ % 2 == 0).sum)
