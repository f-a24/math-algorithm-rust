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
    let mut g = vec![Vec::new(); n + 1]; // g[i] は頂点 i に隣接する頂点のリスト
    for i in 0..m {
        g[a[i]].push(b[i]); // 頂点 a[i] に隣接する頂点として b[i] を追加
        g[b[i]].push(a[i]); // 頂点 b[i] に隣接する頂点として a[i] を追加
    }

    // 深さ優先探索の初期化
    let mut visited = vec![false; n + 1];
    let mut s = VecDeque::new(); // スタック s を定義
    visited[1] = true;
    s.push_back(1); // s に 1 を追加

    // 深さ優先探索
    while s.len() >= 1 {
        let pos = s.pop_front().unwrap(); // s の先頭を調べ、これを取り出す
        for nex in g[pos].to_vec().into_iter() {
            if !visited[nex] {
                visited[nex] = true;
                s.push_back(nex); // s に nex を追加
            }
        }
    }

    // 連結かどうかの判定（answer = true のとき連結）
    let mut answer = true;
    for i in 1..=n {
        if !visited[i] {
            answer = false;
        }
    }
    if answer {
        println!("The graph is connected.");
    } else {
        println!("The graph is not connected.");
    }
}
