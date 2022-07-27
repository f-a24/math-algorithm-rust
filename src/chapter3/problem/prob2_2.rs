use proconio::input;

fn main() {
    input! { n: usize, a: [i32; n] }
    let mut r = lcm(a[0], a[1]);
    for i in 2..n {
        r = lcm(r, a[i]);
    }
    println!("{}", r);
}

fn lcm(a: i32, b: i32) -> i32 {
    (a / gcd(a, b)) * b
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
