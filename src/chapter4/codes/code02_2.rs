use proconio::input;
use std::collections::VecDeque;

fn main() {
    // 入力
    input! { n: usize, q: usize, lrx: [usize; q * 3] }
    let mut in_vec = VecDeque::from(lrx.clone());
    let mut l = Vec::new();
    let mut r = Vec::new();
    let mut x = Vec::new();
    for _ in 0..q {
        l.push(in_vec.pop_front().unwrap());
        r.push(in_vec.pop_front().unwrap());
        x.push(in_vec.pop_front().unwrap());
    }

    // 階差の計算
    let mut b: Vec<i32> = vec![0; n + 2];
    for i in 0..q {
        b[l[i]] += x[i] as i32;
        b[r[i] + 1] -= x[i] as i32;
    }

    // 答えを計算して出力
    let mut answer = String::new();
    for i in 2..n + 1 {
        if b[i] > 0 {
            answer += "<";
        }
        if b[i] == 0 {
            answer += "=";
        }
        if b[i] < 0 {
            answer += ">";
        }
    }
    println!("{}", answer);
}
