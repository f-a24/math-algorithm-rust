use proconio::input;

fn main() {
    // 入力
    input! { a: i32, b: i32, c: i32 }

    // c = 1 のときの場合分け
    if c == 1 {
        println!("No");
        return;
    }

    // 右辺の計算（c の b 乗）
    let mut v = 1;
    for _ in 0..b {
        v *= c;
        if a < v {
            println!("Yes");
            return;
        }
    }

    // 出力（No の場合）
    println!("No");
}
