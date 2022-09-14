use proconio::input;
use std::collections::VecDeque;

fn main() {
    // 入力
    input! { n: usize, q: usize, a: [usize; n], lr: [usize; q * 2] }
    let mut in_vec = VecDeque::from(lr.clone());
    let mut l = Vec::new();
    let mut r = Vec::new();
    for _ in 0..q {
        l.push(in_vec.pop_front().unwrap());
        r.push(in_vec.pop_front().unwrap());
    }

    // 累積和を求める
    let mut b = vec![0; n + 1];
    for i in 0..n {
        b[i + 1] = b[i] + a[i];
    }

    // 答えを計算して出力
    for i in 0..q {
        println!("{}", b[r[i]] - b[l[i] - 1]);
    }
}
