use proconio::input;

fn main() {
    // 入力
    input! { n: i32, mut a: [i32; n] }

    // 配列 a 全体をソートする
    a.sort();

    // 答えを求める
    let mut answer = 0;
    for i in 1..=n {
        answer += a[i as usize - 1] * (-n + 2 * i - 1);
    }

    // 答えの出力
    print!("{}", answer);
}
