use proconio::input;

fn main() {
    // 入力
    input! { n: i32 }

    // 答えを求める
    const MOD: i32 = 1000000007;
    let val = n * (n + 1) / 2;
    println!("{}", val * val % MOD);
}
