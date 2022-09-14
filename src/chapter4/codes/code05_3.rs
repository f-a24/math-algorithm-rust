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

    // 幅優先探索の初期化 (dist[i] = -1 のとき、未到達の白色頂点である）
    let mut dist = vec![-1; n + 1];
    let mut q = VecDeque::new();
    dist[1] = 0;
    q.push_back(1); // q に 1 を追加（操作 1）

    // 幅優先探索
    while !q.is_empty() {
        let pos = q.pop_front().unwrap(); //  q の先頭を調べ、これを取り出す（操作 2, 3）
        for nex in g[pos].to_vec().into_iter() {
            if dist[nex] == -1 {
                dist[nex] = dist[pos] + 1;
                q.push_back(nex); //  q に nex を追加（操作 1）
            }
        }
    }

    //  頂点 1 から各頂点までの最短距離を出力
    for i in 1..=n {
        println!("{}", dist[i]);
    }
}
