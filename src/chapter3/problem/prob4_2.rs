use proconio::input;

fn main() {
    input! { n: usize }

    let mut answer = 0.0;
    for i in 1..=n {
        answer += 1.0 * (n as f64) / (i as f64);
    }

    println!("{}", answer);
}
