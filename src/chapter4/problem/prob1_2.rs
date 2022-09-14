use proconio::input;

fn main() {
    // 入力
    input! { x1: f64, y1: f64, r1: f64, x2: f64, y2: f64, r2: f64 }

    // 円の中心間距離を求める
    let d = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();

    // 答えの出力
    if d < (r1 - r2).abs() {
        println!("1");
    } else if d == (r1 - r2).abs() {
        println!("2");
    } else if d < r1 + r2 {
        println!("3");
    } else if d == r1 + r2 {
        println!("4");
    } else {
        println!("5");
    }
}
