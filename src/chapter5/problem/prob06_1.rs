use proconio::input;

fn main() {
    // 入力
    input! { n: usize, a: [i32; n] }

    //  2^i を求める
    const MOD: i32 = 1000000007;
    let mut power = vec![0; n];
    power[0] = 1;
    for i in 1..n {
        power[i] = (2 * power[i - 1]) % MOD;
    }

    // 答えを求める
    let mut answer = 0;
    for i in 0..n {
        answer += power[i] * a[i];
        answer %= MOD;
    }

    // 出力
    print!("{}", answer);
}
