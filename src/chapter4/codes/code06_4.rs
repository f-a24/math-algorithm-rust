use proconio::input;

fn main() {
    // 入力
    input! { a: usize, b: usize }
    println!("{}", modpow(a, b, 1000000007));
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
