use proconio::input;

fn main() {
    // 入力
    input! { n: usize }

    // N = 2^k-1 の形で表されるかどうか調べる
    let mut flag = false;
    for i in 1..=60 {
        if n == (1 << i) - 1 {
            flag = true;
        }
    }

    // 出力
    if flag {
        println!("Second");
    } else {
        println!("First");
    }
}
