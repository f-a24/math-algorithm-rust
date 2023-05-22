use proconio::input;

fn main() {
    // 入力
    input! { n: i32 }

    if n % 4 == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
