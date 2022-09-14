use proconio::input;

fn main() {
    // 入力 → 配列の初期化
    input! { n: usize };
    let mut f = vec![0; n + 1];

    // F[1], F[2], ..., F[N] を計算する
    for i in 1..=n {
        for j in 1..n / i + 1 {
            f[j * i] += 1;
        }
    }

    // 答えを求める
    let mut answer = 0;
    for i in 1..=n {
        answer += i * f[i]
    }

    println!("{}", answer);
}
