use proconio::input;

fn main() {
    // 入力
    input! { n: i32 }

    if n % 2 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
