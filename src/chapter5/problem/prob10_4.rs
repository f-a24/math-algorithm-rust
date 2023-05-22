use proconio::input;
use std::collections::HashSet;

fn main() {
    // 入力
    input! { n: isize, b: isize }

    // 各桁の積の候補を列挙
    let fm_cand = func(0, 0);

    // m - f(m) = b になるかどうかチェック
    let mut answer = 0;
    for fm in fm_cand {
        let m = fm + b; // 各桁の積から導かれる m の値
        let prod_m = product(m); // 本来の f(m) の値
        if m - prod_m == b && m <= n {
            answer += 1;
        }
    }

    // 出力
    println!("{}", answer);
}

/**
 * 整数 m の各桁の積を返す関数
 */
fn product(mut m: isize) -> isize {
    if m == 0 {
        return 0;
    }
    let mut ans = 1;
    while m >= 1 {
        ans *= m % 10;
        m /= 10;
    }
    ans
}

/**
 * 各桁の積の候補の集合を返す関数
 */
fn func(digit: isize, m: isize) -> HashSet<isize> {
    if digit == 11 {
        return HashSet::from([product(m)]);
    }

    // 次の桁を探索
    // min_value は cur の最後の桁（単調増加にするためには次の桁がそれ以上でなければならない）
    let min_value = m % 10;
    let mut ret = HashSet::new();
    for i in min_value..10 {
        let r = func(digit + 1, m * 10 + i);
        for j in r {
            ret.insert(j);
        }
    }
    ret
}
