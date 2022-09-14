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

    // 出力（g[i].len() は頂点 i に隣接する頂点のリストの大きさ ＝ 次数）
    for i in 1..=n {
        print!("{} : {{", i);
        for j in 0..g[i].len() {
            if j >= 1 {
                print!(",");
            }
            print!("{}", g[i][j]); // g[i][j] は頂点 i に隣接する頂点のうち j+1 番目のもの
        }
        println!("}}");
    }
}
