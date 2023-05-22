use proconio::input;
use std::collections::VecDeque;

fn main() {
    // 入力
    input! { n: usize, m: usize, abc: [usize; m * 3] }
    let mut in_vec = VecDeque::from(abc.clone());
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    for _ in 0..m {
        a.push(in_vec.pop_front().unwrap());
        b.push(in_vec.pop_front().unwrap());
        c.push(in_vec.pop_front().unwrap());
    }

    // 隣接リストの作成 → グラフの辺を追加
    let mut g = vec![Vec::new(); n + 1]; // g[i] は頂点 i に隣接する頂点のリスト
    for i in 0..m {
        g[a[i]].push((b[i], c[i]));
        g[b[i]].push((a[i], c[i]));
    }

    // ダイクストラ法：配列の初期化など
    let mut dist = vec![usize::MAX; n + 1];
    let mut used = vec![false; n + 1];
    let mut q = VecDeque::new();
    dist[1] = 0;
    q.push_back((0, 1));

    // ダイクストラ法：優先度付きキューの更新
    while !q.is_empty() {
        let pos = q.pop_front().unwrap().1;
        if used[pos] {
            continue;
        }
        used[pos] = true;
        for i in g[pos].to_vec().into_iter() {
            let to = i.0;
            let cost = dist[pos] + i.1;
            if dist[to] > cost {
                dist[to] = cost;
                q.push_back((dist[to], to));
            }
        }
    }

    // 答えを出力
    if dist[n] != usize::MAX {
        println!("{}", dist[n]);
    } else {
        println!("-1");
    }
}
