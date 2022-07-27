use proconio::input;

fn main() {
    input! { n: usize, a: [f64; n], b: [f64; n] }

    let mut answer = 0.0;
    for i in 0..n {
        answer += a[i] * (1.0 / 3.0) + b[i] * (2.0 / 3.0)
    }

    println!("{}", answer);
}
