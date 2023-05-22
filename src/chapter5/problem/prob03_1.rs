use proconio::input;

fn main() {
    // 入力
    input! { h: isize, w: isize }

    // 場合分け
    if h == 1 || w == 1 {
        println!("1");
    } else {
        println!("{}", (h * w + 1) / 2);
    }
}
