use proconio::input;

const MOD: usize = 1000000007;

fn main() {
    // 入力 → 累乗の計算（N が 2 以上でなければ正しく動作しないので注意）
    input! { n: usize }
    let a = [[1, 1, 1], [1, 0, 0], [0, 1, 0]];
    let b = power(a, n - 1);

    let answer = (2 * b[2][0] + b[2][1] + b[2][2]) % MOD;
    println!("{}", answer);
}

/**
 * 3×3 行列 A, B の積を返す関数
 */
fn multiply(a: [[usize; 3]; 3], b: [[usize; 3]; 3]) -> [[usize; 3]; 3] {
    let mut c = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
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
fn power(a: [[usize; 3]; 3], n: usize) -> [[usize; 3]; 3] {
    let mut p = a.clone();
    let mut q = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
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
        println!("power: {:?}", p);
        p = multiply(p, p).clone();
    }
    q
}
