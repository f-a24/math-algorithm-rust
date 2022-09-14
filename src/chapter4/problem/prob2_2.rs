use proconio::input;
use std::collections::VecDeque;

fn main() {
    // 入力
    input! { t: usize, n: usize,  lr: [usize; n * 2] }
    let mut in_vec = VecDeque::from(lr.clone());
    let mut l = Vec::new();
    let mut r = Vec::new();
    for _ in 0..n {
        l.push(in_vec.pop_front().unwrap());
        r.push(in_vec.pop_front().unwrap());
    }

    //  階差 B[i] を計算する
    let mut b = vec![0; t + 2];
    for i in 0..n {
        b[l[i]] += 1;
        b[r[i]] -= 1;
    }

    // 累積和 A[i] を計算する
    let mut a = vec![0; t + 2];
    a[0] = b[0];
    for i in 1..t {
        a[i] = a[i - 1] + b[i];
    }

    // 出力
    for i in 0..t {
        println!("{}", a[i]);
    }
}
