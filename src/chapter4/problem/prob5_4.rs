use proconio::input;
use std::collections::VecDeque;

fn main() {
    // 入力
    input! { k: usize }

    // 隣接リストの作成
    let mut g: Vec<Vec<(usize, usize)>> = vec![Vec::new(); k];

    // グラフの辺を追加
    for i in 0..k {
        for j in 0..10 {
            if i == 0 && j == 0 {
                continue;
            }
            g[i].push(((i * 10 + j) % k, j));
        }
    }

    // ダイクストラ法：配列の初期化など
    let mut dist = vec![10_i64.pow(10) as usize; k];
    let mut used = vec![false; k];
    let mut q = VecDeque::new();
    q.push_back((0, 0));

    // ダイクストラ法：優先度付きキューの更新
    while q.len() >= 1 {
        let pos = q.pop_front().unwrap().1;
        if used[pos] == true {
            continue;
        }
        used[pos] = true;
        for i in &g[pos] {
            let mut cost = dist[pos] + i.1;
            if pos == 0 {
                cost = i.1;
            }
            if dist[i.0] > cost {
                dist[i.0] = cost;
                q.push_back((dist[i.0], i.0));
            }
        }
    }

    println!("{}", dist[0]);
}
