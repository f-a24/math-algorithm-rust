use proconio::input;

const MOD: usize = 1000000007;

fn main() {
    // 入力
    input! { x: usize, y: usize }

    // 二項係数の分子と分母を求める（手順 1.／手順 2.）
    let mut bunshi = 1;
    let mut bunbo = 1;
    for i in 1..x + y + 1 {
        bunshi = (bunshi * i) % MOD;
    }
    for i in 1..x + 1 {
        bunbo = (bunbo * i) % MOD;
    }
    for i in 1..y + 1 {
        bunbo = (bunbo * i) % MOD;
    }

    println!("{}", division(bunshi, bunbo, MOD));
}

/**
 * 繰り返し二乗法（p は a**1, a**2, a**4, a**8, ... といった値をとる）
 */
fn modpow(a: usize, b: usize, m: usize) -> usize {
    let mut p = a;
    let mut answer = 1;
    for i in 0..30 {
        if (b & (1 << i)) != 0 {
            answer = (answer * p) % m;
        }
        p = (p * p) % m;
    }
    answer
}

/**
 * division(a, b, m) は a÷b mod m を返す関数
 */
fn division(a: usize, b: usize, m: usize) -> usize {
    (a * modpow(b, m - 2, m)) % m
}
