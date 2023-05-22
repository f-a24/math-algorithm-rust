use std::collections::VecDeque;

use proconio::input;

fn main() {
    // 入力
    input! { n: usize, abc: [f32; n * 3] }
    let mut in_vec = VecDeque::from(abc.clone());
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    for _ in 0..n {
        a.push(in_vec.pop_front().unwrap());
        b.push(in_vec.pop_front().unwrap());
        c.push(in_vec.pop_front().unwrap());
    }

    // 交点を全検索
    let mut answer = 0.0;
    for i in 0..n {
        for j in i + 1..n {
            // 交点を持たない場合
            if a[i] * b[j] == a[j] * b[i] {
                continue;
            }

            // i 番目の直線（条件式の境界）と j 番目の直線（条件式の境界）の交点を求める
            let px = (c[i] * b[j] - c[j] * b[i]) / (a[i] * b[j] - a[j] * b[i]);
            let py = (c[i] * a[j] - c[j] * a[i]) / (b[i] * a[j] - b[j] * a[i]);

            // 座標 (px, py) が N 個の条件すべてを満たすか調べる
            // C++ のコードにおける check 関数に相当
            let mut ret = true;
            for k in 0..n {
                if a[k] * px + b[k] * py > c[k] {
                    ret = false;
                }
            }
            if ret == true {
                answer = (px + py).max(answer);
            }
        }
    }

    // 出力
    print!("{}", answer);
}
