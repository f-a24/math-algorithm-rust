use proconio::input;
use std::collections::VecDeque;

fn main() {
    // 入力
    input! { n: usize, xy: [f64; n * 2] }
    let mut in_vec = VecDeque::from(xy.clone());
    let mut x = Vec::new();
    let mut y = Vec::new();
    for _ in 0..n {
        x.push(in_vec.pop_front().unwrap());
        y.push(in_vec.pop_front().unwrap());
    }

    // 全検索
    let mut answer = 1000000000.0;
    for i in 0..n {
        for j in i + 1..n {
            let dist = ((x[i] - x[j]).powf(2.0) + (y[i] - y[j]).powf(2.0)).sqrt();
            answer = dist.min(answer);
        }
    }

    // 答えの出力
    println!("{}", answer);
}
