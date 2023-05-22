use proconio::input;

fn main() {
    // 入力
    input! { a: i32, b: i32, c: i32 }

    if c - a - b < 0 {
        println!("No");
    } else if 4 * a * b < (c - a - b) * (c - a - b) {
        println!("Yes");
    } else {
        println!("No");
    }
}
