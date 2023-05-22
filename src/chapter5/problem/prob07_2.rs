use proconio::input;
use std::collections::VecDeque;

fn main() {
    // 入力
    input! { n: i32, xy: [i32; n * 2] }
    let mut in_vec = VecDeque::from(xy.clone());
    let mut x = Vec::new();
    let mut y = Vec::new();
    for _ in 0..n {
        x.push(in_vec.pop_front().unwrap());
        y.push(in_vec.pop_front().unwrap());
    }

    // 配列 x, y をソートする
    x.sort();
    y.sort();

    // パーツ 1 の答え（x 座標の差の絶対値の総和）
    let mut part1 = 0;
    // パーツ 2 の答え（y 座標の差の絶対値の総和）
    let mut part2 = 0;

    for i in 1..=n {
        part1 += x[i as usize - 1] * (-n + 2 * i - 1);
        part2 += y[i as usize - 1] * (-n + 2 * i - 1);
    }

    // 答えの出力
    print!("{}", part1 + part2);
}
