use proconio::input;

fn main() {
    // 入力
    input! { n: usize, a: [i32; n - 1], m: usize, b: [usize; m] }

    // 累積和をとる
    let mut s = vec![0; n];
    for i in 1..n {
        s[i] = s[i - 1] + a[i - 1];
    }

    // 答えを求める
    let mut answer = 0;
    for i in 0..m - 1 {
        if b[i] < b[i + 1] {
            answer += s[b[i + 1] - 1] - s[b[i] - 1]
        } else {
            answer += s[b[i] - 1] - s[b[i + 1] - 1]
        }
    }

    // 出力
    println!("{}", answer);
}
