use proconio::input;

fn main() {
    // 入力 → 配列 prime の初期化
    input! { n: usize }
    let mut prime = vec![true; n + 1];

    // エラトステネスのふるい
    let limit = (n as f64).sqrt() as usize;
    for i in 2..=limit {
        if prime[i] {
            for j in (2 * i..=n).step_by(i) {
                prime[j] = false;
            }
        }
    }

    // N 以下の素数を小さい順に出力
    for i in 2..=n {
        if prime[i] {
            print!("{} ", i);
        }
    }
}
