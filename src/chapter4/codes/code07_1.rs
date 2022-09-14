use proconio::input;

const MOD: usize = 1000000000;

fn main() {
    // 入力 → 累乗の計算（N が 2 以上でなければ正しく動作しないので注意）
    input! { n: usize }
    let a = [[1, 1], [1, 0]];
    let b = power(a, n - 1);

    // 答えの計算 → 出力（下から 9 桁目が 0 の場合、最初に 0 を含まない形で出力していることに注意）
    let answer = (b[1][0] + b[1][1]) % MOD;
    println!("{}", answer);
}

/**
 * 2×2 行列 A, B の積を返す関数
 */
fn multiply(a: [[usize; 2]; 2], b: [[usize; 2]; 2]) -> [[usize; 2]; 2] {
    let mut c = [[0, 0], [0, 0]];
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                c[i][j] += a[i][k] * b[k][j];
                c[i][j] %= MOD
            }
        }
    }
    c
}

/**
 * A の n 乗を返す関数
 */
fn power(a: [[usize; 2]; 2], n: usize) -> [[usize; 2]; 2] {
    let mut p = a.clone();
    let mut q = [[0, 0], [0, 0]];
    let mut flag = false;
    for i in 0..60 {
        if (n & (1 << i)) != 0 {
            if flag == false {
                q = p.clone();
                flag = true;
            } else {
                q = multiply(q, p).clone();
            }
        }
        p = multiply(p, p).clone();
    }
    q
}
