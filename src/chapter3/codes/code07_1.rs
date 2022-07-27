use proconio::input;
use std::cmp;

fn main() {
    input! { n: usize, h: [i32; n] }

    let mut dp = vec![0; n];
    for i in 1..n {
        if i == 1 {
            dp[i] = (h[i - 1] - h[i]).abs();
        }
        if i >= 2 {
            let v1 = dp[i - 1] + (h[i - 1] - h[i]).abs();
            let v2 = dp[i - 2] + (h[i - 2] - h[i]).abs();
            dp[i] = cmp::min(v1, v2);
        }
    }

    println!("{}", dp[n - 1]);
}
