use proconio::input;

fn main() {
    // 入力
    input! { n: i32 }

    if n % 4 == 1 {
        println!("{}", 2);
    }
    if n % 4 == 2 {
        println!("{}", 4);
    }
    if n % 4 == 3 {
        println!("{}", 8);
    }
    if n % 4 == 0 {
        println!("{}", 6);
    }
}
