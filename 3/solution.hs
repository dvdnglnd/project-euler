sieve :: Integral t => [t] -> [t]
sieve [] = []
sieve (p:xs) = p : sieve [x | x <- xs, x `mod` p > 0]

primes :: [Int]
primes = sieve [2..]

primeFactors :: Int -> [Int]
primeFactors n = [x | x <- takeWhile (< n) primes, n `mod` x == 0]

-- Coudn't get sieve version fast enough???

floorRoot = floor . sqrt . fromIntegral

isPrime n
  | n `mod` 2 == 0 && n > 2 = False
  | otherwise = all (\x -> n `mod` x /= 0) [x | x <- [3,5..(floor . sqrt . fromIntegral) n]]

pFactors n = filter (\x -> n `mod` x == 0 && isPrime x) [1..(floorRoot n)]

main::IO ()
main = print $ maximum (pFactors 600851475143)
