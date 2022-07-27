use proconio::input;

fn main() {
    input! { n: usize, a: [i32; n] }
    let mut r = gcd(a[0], a[1]);
    for i in 2..n {
        r = gcd(r, a[i]);
    }
    println!("{}", r);
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
