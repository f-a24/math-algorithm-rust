use proconio::input;

const MOD: usize = 1000000007;
const LIMIT: usize = 200000;

fn main() {
    // 入力
    input! { n: usize, a: [usize; n] }

    // 配列 fact の初期化（fact[i] は i の階乗を 1000000007 で割った余り）
    let mut fact = vec![0; LIMIT + 1];
    fact[0] = 1;
    for i in 1..=LIMIT {
        fact[i] = fact[i - 1] * i % MOD;
    }

    // 答えを求める
    let mut answer = 0;
    for i in 0..n {
        answer += a[i] * ncr(n - 1, i, &fact);
        answer %= MOD;
    }

    // 答えの出力
    print!("{}", answer);
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

// division(a, b, m) は a÷b mod m を返す関数
fn division(a: usize, b: usize, m: usize) -> usize {
    (a * modpow(b, m - 2, m)) % m
}

// ncr は n! を r! × (n-r)! で割った値
fn ncr(n: usize, r: usize, fact: &Vec<usize>) -> usize {
    division(fact[n], fact[r] * fact[n - r] % MOD, MOD)
}
