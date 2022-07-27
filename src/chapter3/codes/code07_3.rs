use proconio::input;
use std::cmp::max;
use std::collections::VecDeque;

fn main() {
    input! { n: usize, w: usize, mut in_w: [i32; n], mut in_v: [i32; n] }
    in_w.append(&mut in_v);
    let mut in_vec = VecDeque::from(in_w.clone());

    let mut vec_w = Vec::new();
    let mut vec_v = Vec::new();
    for _ in 0..n {
        vec_w.push(in_vec.pop_front().unwrap());
        vec_v.push(in_vec.pop_front().unwrap());
    }

    let mut dp = vec![vec![0; w + 1]; n + 1];
    let inf = 1000000000;
    for i in 1..=w {
        dp[0][i] = -inf;
    }

    for i in 1..=n {
        for j in 0..=w {
            if (j as i32) < vec_w[i - 1] {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = max(
                    dp[i - 1][j],
                    dp[i - 1][j - (vec_w[i - 1] as usize)] + vec_v[i - 1],
                );
            }
        }
    }

    let mut answer = 0;
    for i in 0..=w {
        answer = max(answer, dp[n][i]);
    }
    println!("{}", answer);
}
