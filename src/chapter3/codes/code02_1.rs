use proconio::input;
use std::cmp;

fn main() {
    input! { a: i32, b: i32 }
    println!("{}", gcd(a, b));
}

fn gcd(a: i32, b: i32) -> i32 {
    let mut ans = 0;
    for i in 1..=cmp::min(a, b) {
        if a % i == 0 && b % i == 0 {
            ans = i
        }
    }
    ans
}
