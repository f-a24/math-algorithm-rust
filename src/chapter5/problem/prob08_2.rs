use proconio::input;

fn main() {
    // 入力
    input! { n: i32 }

    // 出力
    println!("{}", n * (n - 1) / 2);
}
