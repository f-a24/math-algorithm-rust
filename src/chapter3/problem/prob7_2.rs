use proconio::input;
use std::cmp::max;

fn main() {
    input! { n: usize, a: [usize; n] }

    let mut dp1 = vec![0; n + 1];
    let mut dp2 = vec![0; n + 1];

    for i in 1..=n {
        dp1[i] = dp2[i - 1] + a[i - 1];
        dp2[i] = max(dp1[i - 1], dp2[i - 1]);
    }

    println!("{}", max(dp1[n], dp2[n]));
}
