use proconio::input;

fn main() {
    // 入力
    input! { n: i32, a: [i32; n] }

    // 答えを求める
    let mut answer = 0;
    for i in 0..n {
        // コード 5.7.1 (C++) では A[i] * (-N + 2 * i - 1) となっていますが、ここでは i = 0 から始まるので -N + 2 * i + 1 が掛けられます。
        answer += a[i as usize] * (-n + 2 * i + 1);
    }

    // 答えの出力
    print!("{}", answer);
}
