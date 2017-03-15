#[macro_use] extern crate itertools;

fn is_palindrome(n: u32) -> bool {
    let s = n.to_string();
    let chars = s.chars();
    if chars == chars.rev() {
        true
    }
    false
}

struct Pair<'a> {
    x: 'a,
    y: 'a,
}

impl Iterator for Pair {
    type Item = 

fn product<'a, I>(xs: I) -> Iterator<'a> where I: Iterator<Item=&'a> {
    for x in xs.iter() {
        for y in xs.iter() {
            
}

fn main() {
    let nums = 10..100;
    let pairs = iproduct!(nums, nums);
    let result = pairs.map(|x, y| x * y)
        .filter(|&x| is_palindrome(x)).max();
    println!("{}", result.to_string())
}
