use proconio::input;
use std::cmp::{max, min};

fn main() {
    // 入力
    input! { n: i32, k: i32 }

    // 事象 b の個数 yojishou を数える
    let mut yojishou = 0;
    for a in 1..=n {
        let l = max(1, a - (k - 1));
        let r = min(n, a + (k - 1));
        for b in l..=r {
            for c in l..=r {
                if i32::abs(b - c) <= k - 1 {
                    yojishou += 1;
                }
            }
        }
    }

    // 答えの出力
    println!("{}", n.pow(3) - yojishou);
}
