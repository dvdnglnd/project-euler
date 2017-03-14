fn round_sqrt(n: u64) -> u64 {
    (n as f64).sqrt() as u64 + 1
}

fn is_prime(n: u64) -> bool {
    if (n%2 == 0) & (n > 2) {
        return false;
    } else {
        (3..round_sqrt(n)).all(|x| n % x != 0)
    }
}

fn largest_factor(n: u64) -> u64 {
    (1..(round_sqrt(n))).filter(|&x| (n%x == 0) & is_prime(x)).max().unwrap_or(0)
}

fn main() {
    assert!(largest_factor(13195) == 29);
    let result = largest_factor(600851475143);
    println!("{}", result.to_string())
}
