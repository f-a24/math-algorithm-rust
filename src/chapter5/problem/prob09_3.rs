use proconio::input;
use std::collections::VecDeque;

fn main() {
    // 入力
    // a は (終了時刻, 開始時刻) になっていることに注意 [終了時刻の昇順にソートするため]
    input! { n: usize, ab: [usize; n * 2] }
    let mut in_vec = VecDeque::from(ab.clone());
    let mut vec_a = Vec::new();
    for _ in 0..n {
        let a = in_vec.pop_front().unwrap();
        let b = in_vec.pop_front().unwrap();
        vec_a.push([b, a]);
    }

    // ソート
    vec_a.sort();

    // 終了時刻が最も早いものを選び続ける
    let mut current_time = 0;
    let mut answer = 0;
    for i in 0..n {
        if current_time <= vec_a[i][1] {
            current_time = vec_a[i][0];
            answer += 1;
        }
    }

    // 出力
    print!("{}", answer);
}
