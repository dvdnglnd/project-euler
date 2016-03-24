fn main() {
    assert_eq!(23, sum3_or_5(10));
    println!("{}", sum3_or_5(1000).to_string());
}

fn sum3_or_5(n: u32) -> u32 {
    (1..n).filter(|x| x % 3 == 0 || x % 5 == 0).fold(0, |sum, x| sum + x)
}
