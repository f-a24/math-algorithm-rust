use proconio::input;
use std::collections::VecDeque;

fn main() {
    // 入力
    input! { n: usize, m: usize, ab: [usize; m * 2] }
    let mut ab_vec = VecDeque::from(ab.clone());
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..m {
        a.push(ab_vec.pop_front().unwrap());
        b.push(ab_vec.pop_front().unwrap());
    }

    // 隣接リストの作成
    let mut g = vec![Vec::new(); n + 1];
    for i in 0..m {
        g[a[i]].push(b[i]);
        g[b[i]].push(a[i]);
    }

    // 答えを求める
    let mut answer = 0;
    for i in 1..=n {
        let mut cnt = 0;
        for j in &g[i] {
            if j < &i {
                cnt += 1;
            }
        }
        // 自分自身より番号が小さい隣接頂点が 1 つであれば answer に 1 を加算する
        if cnt == 1 {
            answer += 1;
        }
    }

    println!("{}", answer);
}
