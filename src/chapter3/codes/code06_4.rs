use proconio::input;

fn main() {
    input! { a: f64, b: f64 }
    println!("{}", gcd(a, b));
}

fn gcd(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        return a;
    }
    gcd(b, a % b)
}
