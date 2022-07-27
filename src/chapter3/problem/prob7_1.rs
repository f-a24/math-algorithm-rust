use proconio::input;

fn main() {
    input! { n: usize, s: usize, a: [usize; n] }

    let mut dp = vec![vec![true; s + 1]; n + 1];
    for i in 1..=s {
        dp[0][i] = false;
    }

    for i in 1..=n {
        for j in 0..=s {
            if j < a[i - 1] {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i - 1][j] || dp[i - 1][j - a[i - 1]];
            }
        }
    }

    println!("{}", if dp[n][s] { "Yes" } else { "No" });
}
