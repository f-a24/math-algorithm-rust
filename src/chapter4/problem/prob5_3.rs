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
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    for i in 0..m {
        g[a[i]].push(b[i]);
        g[b[i]].push(a[i]);
    }

    // 深さ優先探索を行う関数
    fn dfs(pos: usize, g: Vec<Vec<usize>>, color: &mut Vec<usize>) {
        for i in g[pos].iter() {
            if (*color)[*i] == 0 {
                // color[pos] = 1 のとき 2、color[pos] = 2 のとき 1
                (*color)[*i] = 3 - color[pos];
                dfs(*i, g.to_vec(), color);
            }
        }
    }

    // 深さ優先探索
    let mut color = vec![0; n + 1];
    for i in 1..=n {
        if color[i] == 0 {
            // 頂点 i は白である（まだ探索されていない連結成分である）
            color[i] = 1;
            dfs(i, g.to_vec(), &mut color);
        }
    }

    // 二部グラフかどうかの判定
    let mut answer = true;
    for i in 0..m {
        if color[a[i]] == color[b[i]] {
            answer = false;
        }
    }

    if answer {
        println!("Yes");
    } else {
        println!("No");
    }
}
