use proconio::input;

fn main() {
    input! { a: i32, b: i32 }
    println!("{}", gcd(a, b));
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a >= 1 && b >= 1 {
        if a < b {
            b %= a;
        } else {
            a %= b
        }
    }
    if a >= 1 {
        return a;
    }
    b
}
