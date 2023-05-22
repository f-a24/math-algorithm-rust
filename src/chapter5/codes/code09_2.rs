use proconio::input;
use std::{cmp::min, collections::VecDeque};

fn main() {
    // 入力
    input! { n: usize, lr: [usize; n * 2] }
    let mut in_vec = VecDeque::from(lr.clone());
    let mut l = Vec::new();
    let mut r = Vec::new();
    for _ in 0..n {
        l.push(in_vec.pop_front().unwrap());
        r.push(in_vec.pop_front().unwrap());
    }

    const INF: usize = 1000000;
    let mut current_time = 0;
    let mut answer = 0;

    loop {
        let mut min_endtime = INF;
        for i in 0..n {
            if l[i] >= current_time {
                min_endtime = min(min_endtime, r[i]);
            }
        }
        if min_endtime == INF {
            break;
        }
        current_time = min_endtime;
        answer += 1;
    }

    // 答えの出力
    print!("{}", answer);
}
