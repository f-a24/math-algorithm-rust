use proconio::{input, marker::Chars};
use std::collections::VecDeque;

fn main() {
    // 入力
    input! { h: usize, w: usize, sx: usize, sy: usize, gx: usize, gy: usize, c: [Chars; h] }
    let start = (sx - 1) * w + (sy - 1);
    let goal = (gx - 1) * w + (gy - 1);

    // 隣接リストの作成
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); h * w];

    // 横方向の辺 [(i, j) - (i, j+1)] をグラフに追加
    for i in 0..h {
        for j in 0..w - 1 {
            if c[i][j] == '.' && c[i][j + 1] == '.' {
                let idx1 = i * w + j; // マス (i, j) の頂点番号
                let idx2 = i * w + (j + 1); // マス (i, j + 1) の頂点番号
                g[idx1].push(idx2);
                g[idx2].push(idx1);
            }
        }
    }

    // 縦方向の辺 [(i, j) - (i+1, j)] をグラフに追加
    for i in 0..h - 1 {
        for j in 0..w {
            if c[i][j] == '.' && c[i + 1][j] == '.' {
                let idx1 = i * w + j; // マス (i, j) の頂点番号
                let idx2 = (i + 1) * w + j; // マス (i + 1, j) の頂点番号
                g[idx1].push(idx2);
                g[idx2].push(idx1);
            }
        }
    }

    // 幅優先探索の初期化 (dist[i] = -1 のとき、未到達の白色頂点である）
    let mut dist = vec![-1; h * w];
    let mut q = VecDeque::new(); // キュー Q を定義する
    dist[start] = 0;
    q.push_back(start); // Q に start を追加

    // 幅優先探索
    while !q.is_empty() {
        let pos = q.pop_front().unwrap(); // Q の先頭を調べ、これを取り出す
        for nex in &g[pos] {
            if dist[*nex] == -1 {
                dist[*nex] = dist[pos] + 1;
                q.push_back(*nex); // Q に nex を追加
            }
        }
    }

    println!("{}", dist[goal]);
}
