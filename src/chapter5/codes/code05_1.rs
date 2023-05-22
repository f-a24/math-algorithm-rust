use proconio::input;
use std::cmp::max;

fn main() {
    // 入力
    input! { a: isize, b: isize, c: isize, d: isize }

    // 答えの出力
    println!("{}", max(max(a * c, a * d), max(b * c, b * d)));
}
