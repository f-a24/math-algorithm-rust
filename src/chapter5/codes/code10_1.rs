use proconio::input;

fn main() {
    // 入力
    input! { a: f32, b: f32, c: f32 }

    if a.sqrt() + b.sqrt() < c.sqrt() {
        println!("Yes");
    } else {
        println!("No");
    }
}
