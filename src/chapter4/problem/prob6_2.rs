use proconio::input;

const MOD: usize = 1000000007;

fn main() {
    // 入力
    input! { n: usize }

    // 答えの計算
    let v = modpow(4, n + 1, MOD) - 1;
    println!("{}", division(v, 3, MOD));
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
