use proconio::input;

fn main() {
    // 入力
    input! { a: i32, b: i32, c: i32 }

    // 計算
    const MOD: i32 = 998244353;
    let d = a * (a + 1) / 2;
    let e = b * (b + 1) / 2;
    let f = c * (c + 1) / 2;

    // 答えを出力
    println!("{}", (d * e * f) % MOD);
}
